from mimesis.random import Random
import pandas as pd

    
# Parameters
total_rows = 1_000_000
chunk_size = 100_000
file_name = 'mimesis_bench.parquet'


def generate_chunk(random, chunk_size):
    return [random.custom_code(mask="@@@@@@@@@@") for i in range(chunk_size)]

if __name__ == '__main__':
    random = Random()

    # Process and write in chunks
    for i in range(0, total_rows, chunk_size):
        chunk = generate_chunk(random, min(chunk_size, total_rows - i))
        df_chunk = pd.DataFrame(chunk, columns=["name"])
        df_chunk.to_parquet(file_name, engine='fastparquet', index=False, append=(i != 0))
