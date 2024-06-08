import random
import json
import os

# Q-Table to store state-action values
Q_TABLE = {}
LEARNING_RATE = 0.1
DISCOUNT_FACTOR = 0.95
EPSILON = 0.1  # Exploration factor

# Load Q-Table if it exists, or initialize a new one if it fails
q_table_file = 'q_table.json'
try:
    if os.path.exists(q_table_file):
        with open(q_table_file, 'r') as f:
            Q_TABLE = json.load(f)
except json.JSONDecodeError:
    print("Failed to load Q-table. Initializing new Q-table.")
    Q_TABLE = {}

def get_state_action_value(state, action):
    return Q_TABLE.get(state, {}).get(action, 0.0)

def set_state_action_value(state, action, value):
    if state not in Q_TABLE:
        Q_TABLE[state] = {}
    Q_TABLE[state][action] = value

def choose_action(state):
    if random.uniform(0, 1) < EPSILON:
        return random.choice(['up', 'down', 'left', 'right'])
    q_values = [get_state_action_value(state, a) for a in ['up', 'down', 'left', 'right']]
    max_q_value = max(q_values)
    actions_with_max_q_value = [a for a, q in zip(['up', 'down', 'left', 'right'], q_values) if q == max_q_value]
    return random.choice(actions_with_max_q_value)

def update_q_table(state, action, reward, next_state):
    current_q = get_state_action_value(state, action)
    max_next_q = max([get_state_action_value(next_state, a) for a in ['up', 'down', 'left', 'right']])
    new_q = current_q + LEARNING_RATE * (reward + DISCOUNT_FACTOR * max_next_q - current_q)
    set_state_action_value(state, action, new_q)
    # Save Q-Table
    with open(q_table_file, 'w') as f:
        json.dump(Q_TABLE, f)

def compute_action(state):
    return choose_action(state)

def learn(state, action, reward, next_state):
    update_q_table(state, action, reward, next_state)
