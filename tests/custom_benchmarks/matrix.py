MSIZE = 64

def c_matrix(name, values):
    lines = []
    lines.append('__attribute__((section(".data")))')
    lines.append(f'unsigned int {name}[MSIZE][MSIZE] = {{')

    for row in values:
        row_str = ", ".join(str(x) for x in row)
        lines.append(f"  {{{row_str}}},")

    lines.append("};")
    return "\n".join(lines)

def main():
    A = [
        [i * MSIZE + j for j in range(MSIZE)]
        for i in range(MSIZE)
    ]

    B = [
        [((i + 1) << 16) + (j + 1) for j in range(MSIZE)]
        for i in range(MSIZE)
    ]

    with open("mmm_matrices.h", "w") as f:
        f.write("#ifndef MMM_MATRICES_H\n")
        f.write("#define MMM_MATRICES_H\n\n")


        f.write(c_matrix("A", A))
        f.write("\n\n")
        f.write(c_matrix("B", B))
        f.write("\n\n")

        f.write("#endif\n")

if __name__ == "__main__":
    main()