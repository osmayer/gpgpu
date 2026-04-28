SIZE = 512

def pixel_expr(r, g, b):
    return f"{{{r}, {g}, {b}}}"

def main():
    with open("data_arrays.h", "w") as f:
        f.write("#ifndef DATA_ARRAYS_H\n")
        f.write("#define DATA_ARRAYS_H\n\n")

        f.write(f"#define SIZE {SIZE}\n\n")

        f.write("typedef struct {\n")
        f.write("    unsigned char r;\n")
        f.write("    unsigned char g;\n")
        f.write("    unsigned char b;\n")
        f.write("} pixel_t;\n\n")

        f.write('__attribute__((section(".data")))\n')
        f.write("pixel_t image[SIZE][SIZE] = {\n")

        for y in range(SIZE):
            row = []
            for x in range(SIZE):
                r = (3 * x + 5 * y) & 0xff
                g = (7 * x + 11 * y) & 0xff
                b = (13 * x + 17 * y) & 0xff
                row.append(pixel_expr(r, g, b))

            f.write("    {" + ", ".join(row) + "},\n")

        f.write("};\n\n")
        f.write("#endif\n")

if __name__ == "__main__":
    main()
