# Z80 CPU Emulator in Rust 🦀

**A complete, educational Z80 emulator built step-by-step using Test-Driven Development**

## Copyright and licensing
Copyright (c) 2025, Synthonyx Technologies Ltd

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>

## 🎯 What We're Building

This project creates a fully functional Z80 CPU emulator in Rust, capable of running real CP/M operating systems and programs. It's designed as an **educational journey** through systems programming, CPU architecture, and emulation.

### Final Goals

By the end of this project, you'll have built:

- ✅ **Complete Z80 CPU Emulator** - All documented instructions and behaviors
- ✅ **Memory System** - RAM, ROM, and I/O port handling  
- ✅ **CP/M 2.2 & 3.0 Support** - Boot and run real operating systems
- ✅ **Built-in Z80 Assembler** - Assemble programs and even CP/M itself
- ✅ **File Loaders** - Load `.COM`, `.BIN`, and `.ASM` files
- ✅ **WASM Compatible** - Run in browsers or sandboxed environments

## 🚀 Why This Project?

The Z80 is the **perfect CPU for learning emulation**:

- **Simple but complete** - 8-bit architecture that's easy to understand
- **Rich instruction set** - Covers all fundamental CPU concepts
- **Historical significance** - Powered countless computers and game systems
- **Real software** - Run actual operating systems and programs
- **Great documentation** - Extensive resources and test programs available

## 📚 Learning Approach

This project uses **Test-Driven Development (TDD)** with incremental steps:

1. **Write tests first** - Define behavior before implementation
2. **Small, focused steps** - Each step builds one specific feature
3. **Learn by doing** - Understand concepts through hands-on coding
4. **Real-world validation** - Test with actual Z80 programs

## 🗂️ Project Structure

### Branches
Each step has its own branch for easy navigation:
```
main           - Final completed emulator
step1          - CPU state and initialization  
step2          - Basic register operations
step3          - Memory system
...
stepN          - Complete emulator
```

### Documentation
Each step includes detailed documentation:
```
steps/
├── 1.md       - CPU state and register initialization
├── 2.md       - Register pair operations and flag handling
├── 3.md       - Memory system and address bus
├── ...        - Progressive learning materials
└── N.md       - Final integration and CP/M boot
```

## 🏁 Getting Started

### Prerequisites
- Rust (latest stable version)
- Basic understanding of programming concepts
- Curiosity about how computers work! 

### Follow Along

1. **Clone the repository**
   ```bash
   git clone https://github.com/arjanvaneersel/z80e
   cd z80e
   ```

2. **Start with Step 1**
   ```bash
   git checkout step1
   ```

3. **Read the step documentation**
   ```bash
   cat steps/1.md
   ```

4. **Run the tests (they should fail initially)**
   ```bash
   cargo test
   ```

5. **Implement the code to make tests pass**

6. **Move to the next step**
   ```bash
   git checkout step2
   cat steps/2.md
   ```

### Self-Paced Learning

- Each step builds on the previous one
- Tests guide your implementation
- Documentation explains the "why" behind each decision
- No step is too large - progress incrementally

## 🎓 What You'll Learn

### Systems Programming Concepts
- CPU architecture and instruction sets
- Memory management and address spaces
- Interrupt handling and I/O systems
- Binary formats and program loading

### Rust Skills  
- Struct design and ownership
- Error handling patterns
- Test-driven development
- Module organization
- Performance considerations

### Computer History
- Evolution of 8-bit computing
- CP/M operating system architecture
- Assembly language programming
- Emulation techniques

## 🌟 Educational Philosophy

This project believes that **building is the best way to understand**. Rather than just reading about CPUs, you'll create one. Rather than just using an emulator, you'll write one.

Each step is designed to:
- Introduce one concept at a time
- Provide immediate feedback through tests
- Explain the historical and technical context
- Build confidence through working code

## 🤝 Contributing

This is an educational project! Contributions welcome:

- **Improve documentation** - Make explanations clearer
- **Add test cases** - More validation is always good
- **Fix bugs** - Help other learners succeed
- **Suggest improvements** - Better ways to teach concepts

## 📖 Resources

### Z80 Documentation
- [Z80 CPU User Manual](http://www.zilog.com/docs/z80/um0080.pdf)
- [Sean Young's Z80 Guide](http://www.msxnet.org/tech/Z80/z80undoc.txt)
- [ClrHome Z80 Reference](http://clrhome.org/table/)

### CP/M Resources
- [CP/M 2.2 Documentation](http://www.cpm.z80.de/)
- [Digital Research Archives](http://www.digitalresearch.biz/)

### Emulation Guides
- [Emulator 101](http://emulator101.com/)
- [Computer Archeology](http://computerarcheology.com/)

## 🎉 Success Stories

When you complete this project, you'll have:

- Deep understanding of CPU architecture
- Solid Rust programming skills  
- A working emulator running real software
- Foundation for emulating other systems
- Confidence in systems programming

## 🙏 Acknowledgments

Inspired by the countless educators, emulator authors, and computer enthusiasts who preserve computing history and make it accessible to new generations.

---

**Ready to build a CPU? Start with `git checkout step1` and let's begin!** 🚀