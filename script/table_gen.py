# Generate rust code for JPEG general purpose hash table
# Place your file's path in this variable
hash_table_path =  "hashtable.txt"

def sysmblo2u8(sysmbol: str) -> int:
    runlength, size = sysmbol.split('/')
    heigh_bits = int(runlength, base=16)
    low_bits = int(size, base=16)

    return (heigh_bits << 4) + low_bits

if __name__ == "__main__":
    with open(hash_table_path, "r") as f:
        for line in f:
            data = line.split()
            if len(data) == 3:
                sysmbol, length, codeword = data[0], data[1], data[2]
                output = "table.insert({}, Bits::new({}, 0b{}));".format(sysmblo2u8(sysmbol), length, codeword)
                print(output)
