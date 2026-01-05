def read_null_terminated(file_path):
    with open(file_path, 'rb') as f:
        cur_string = bytearray()

        while True:
            char = f.read(1)
            
            if not char: # EOF
                if cur_string:
                    yield cur_string.decode('utf-8')
                break

            if char == b'\x00':
                yield cur_string.decode('utf-8')
                cur_string = bytearray()
            else:
                cur_string.extend(char)

if __name__ == "__main__":
    for index, string in enumerate(read_null_terminated("FlfGameWorkROString.bin")):
        print(f"{index}: {string}")
        