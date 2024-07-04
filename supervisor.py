import subprocess
import time
import os
import re
import json
from datetime import datetime

# Define the paths
rust_project_dir = r'C:\Users\fabio\Desktop\uni\MachineLearning\squaro_py_in_ru\squaro'
q_table_path = os.path.join(rust_project_dir, 'q_table.json')
log_file = os.path.join(rust_project_dir, 'training_log.txt')

# Function to start Rust program
def run_rust_program(duration_minutes):
    process = subprocess.Popen(['cargo', 'run'], cwd=rust_project_dir, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    # Run for the specified duration
    time.sleep(duration_minutes * 60)
    
    # Terminate the Rust program
    process.terminate()
    try:
        stdout, stderr = process.communicate(timeout=10)
    except subprocess.TimeoutExpired:
        process.kill()
        stdout, stderr = process.communicate()

    # Parse points from the console print
    points = parse_points_from_output(stdout.decode('utf-8'))
    return points

# Function to parse points from the console
def parse_points_from_output(output):
    matches = re.findall(r'Current score:\s*(\d+)', output)
    if matches:
        return int(matches[-1])  # Return the last matched score
    return 0

# Function to get the size of the Q-table
def get_q_table_size():
    if os.path.exists(q_table_path):
        try:
            with open(q_table_path, 'r') as file:
                data = json.load(file)
                return len(data)
        except (json.JSONDecodeError, ValueError):
            # Return 0 if the file is empty or contains invalid JSON
            return 0
    return 0

# Function to log the results
def log_results(duration_minutes, points, trained, q_table_size, iteration_count):
    with open(log_file, 'a') as file:
        timestamp = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
        status = 'with trained data' if trained else 'without trained data'
        file.write(f'{timestamp} - {duration_minutes} minutes training: {points} points ({status}), Q-table size: {q_table_size}, Q-table iterations: {iteration_count}\n')

# Function to delete Q-table
def delete_q_table():
    if os.path.exists(q_table_path):
        os.remove(q_table_path)

# Main function to run supervisor
def main():
    duration_minutes = 5
    iterations_no_qtable = 15  # 10 iterations without deleting the Q-table
    iterations_with_qtable = 15  # 5 iterations with Q-table intact
    iteration_count = 0  # Initialize the iteration count

    # Run iterations without deleting the Q-table (continue learning)
    for i in range(iterations_with_qtable):
        points = run_rust_program(duration_minutes)
        q_table_size = get_q_table_size()
        log_results(duration_minutes, points, trained=True, q_table_size=q_table_size, iteration_count=iteration_count)
        print(f'Iteration {i+1}/{iterations_with_qtable} (with Q-table intact): {points} points, Q-table size: {q_table_size}, Q-table iterations: {iteration_count}')
        iteration_count += 1


    # Run iterations with deleting the Q-table (fresh start)
    for i in range(iterations_no_qtable):
        delete_q_table()  # Delete the Q-table to start fresh
        points = run_rust_program(duration_minutes)
        q_table_size = get_q_table_size()
        log_results(duration_minutes, points, trained=False, q_table_size=q_table_size, iteration_count=iteration_count)
        print(f'Iteration {i+1}/{iterations_no_qtable} (fresh start): {points} points, Q-table size: {q_table_size}, Q-table iterations: {iteration_count}')
        iteration_count += 1

    
if __name__ == '__main__':
    main()
