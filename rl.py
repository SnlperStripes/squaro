import random
import json
import os
import atexit
from collections import OrderedDict
import zlib
import base64
from concurrent.futures import ThreadPoolExecutor

# Helper functions for compressing and decompressing state representations
def compress_state(state):
    state_bytes = json.dumps(state).encode('utf-8')
    compressed = zlib.compress(state_bytes)
    return base64.b64encode(compressed).decode('utf-8')

def decompress_state(compressed_state):
    state_bytes = base64.b64decode(compressed_state)
    decompressed = zlib.decompress(state_bytes)
    return json.loads(decompressed.decode('utf-8'))

# LRU Cache Implementation
class LRUCache:
    def __init__(self, capacity=4194304):
        self.capacity = capacity
        self.cache = OrderedDict()
        self.usage_counts = OrderedDict()
        self.initial_load = True  # Flag to prevent pruning during initial load

    def get(self, key):
        if key in self.cache:
            self.cache.move_to_end(key)
            self.usage_counts[key] += 1
            return self.cache[key]
        return None

    def put(self, key, value):
        if key in self.cache:
            self.cache.move_to_end(key)
        self.cache[key] = value
        self.usage_counts[key] = self.usage_counts.get(key, 0) + 1
        if len(self.cache) > self.capacity:
            self.prune()
    
    def prune(self):
        if len(self.cache) <= self.capacity or self.initial_load:
            return
        sorted_usage = sorted(self.usage_counts.items(), key=lambda item: item[1])
        num_items_to_prune = len(sorted_usage) // 10  # Remove the least used 10% of items
        for key, _ in sorted_usage[:num_items_to_prune]:
            if key in self.cache:
                del self.cache[key]
                del self.usage_counts[key]
            if len(self.cache) <= self.capacity:
                break

Q_TABLE = LRUCache(capacity=4194304)
LEARNING_RATE = 0.1
DISCOUNT_FACTOR = 0.95
EPSILON = 0.1
MIN_EPSILON = 0.01
EPSILON_DECAY = 0.995
ACTION_PERSISTENCE = 10

q_table_file = 'q_table.json'
try:
    if os.path.exists(q_table_file):
        with open(q_table_file, 'r') as f:
            data = json.load(f)
            for key, value in data.items():
                decompressed_key = decompress_state(key)
                Q_TABLE.put(decompressed_key, value)
        Q_TABLE.initial_load = False  # Disable initial load flag after loading
except (json.JSONDecodeError, zlib.error, base64.binascii.Error) as e:
    print(f"Failed to load Q-table: {e}. Initializing new Q-table.")
    Q_TABLE = LRUCache(capacity=4194304)

current_action = None
action_counter = 0
save_counter = 0
SAVE_INTERVAL = 1000
PRUNE_INTERVAL = 10000
prune_counter = 0

ACTIONS = ['up', 'down', 'left', 'right']

def get_state_action_value(state, action):
    value = Q_TABLE.get(state)
    if value:
        return value.get(action, 0.0)
    return 0.0

def set_state_action_value(state, action, value):
    state_actions = Q_TABLE.get(state) or {}
    state_actions[action] = round(value, 4)  # Quantize Q-value to 4 decimal places
    Q_TABLE.put(state, state_actions)

def choose_action(state):
    global current_action, action_counter
    if action_counter > 0:
        action_counter -= 1
        return current_action

    if random.uniform(0, 1) < EPSILON:
        action = random.choice(ACTIONS)
    else:
        q_values = [get_state_action_value(state, a) for a in ACTIONS]
        max_q_value = max(q_values)
        actions_with_max_q_value = [a for a, q in zip(ACTIONS, q_values) if q == max_q_value]
        action = random.choice(actions_with_max_q_value)
    
    current_action = action
    action_counter = ACTION_PERSISTENCE
    return action

def update_q_table(state, action, reward, next_state):
    global save_counter, prune_counter
    current_q = get_state_action_value(state, action)
    max_next_q = max([get_state_action_value(next_state, a) for a in ACTIONS])
    new_q = current_q + LEARNING_RATE * (reward + DISCOUNT_FACTOR * max_next_q - current_q)
    set_state_action_value(state, action, new_q)
    save_counter += 1
    prune_counter += 1
    if save_counter >= SAVE_INTERVAL:
        save_q_table()
        save_counter = 0
    if prune_counter >= PRUNE_INTERVAL:
        Q_TABLE.prune()
        prune_counter = 0

def compute_action(state):
    return choose_action(state)

def learn(state, action, reward, next_state):
    update_q_table(state, action, reward, next_state)

def decay_epsilon():
    global EPSILON
    EPSILON = max(MIN_EPSILON, EPSILON * EPSILON_DECAY)

# Define the thread pool executor
executor = ThreadPoolExecutor(max_workers=1)

# Define the function to save the Q-table asynchronously
def async_save_q_table():
    with open(q_table_file, 'w') as f:
        try:
            compressed_data = {compress_state(k): v for k, v in Q_TABLE.cache.items()}
            json.dump(compressed_data, f)
        except (zlib.error, base64.binascii.Error) as e:
            print(f"Failed to save Q-table: {e}")

# Modify the save_q_table function to use the executor
def save_q_table():
    executor.submit(async_save_q_table)

# Register the save_q_table function to be called on program exit
atexit.register(save_q_table)
