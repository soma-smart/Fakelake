import faker
import pandas as pd

    
# Parameters
total_rows = 1_000_000
chunk_size = 100_000
file_name = 'faker_bench.parquet'


def generate_chunk(fake, index, chunk_size):
    return [fake.bothify(text='??????????') for i in range(chunk_size)]

if __name__ == '__main__':
    fake = faker.Faker()

    # Process and write in chunks
    for i in range(0, total_rows, chunk_size):
        chunk = generate_chunk(fake, i * chunk_size, min(chunk_size, total_rows - i))
        df_chunk = pd.DataFrame(chunk, columns=["name"])
        df_chunk.to_parquet(file_name, engine='fastparquet', index=False, append=(i != 0))
