## optframe-rust

Welcome to OptFrame project in Rust.

### What is OptFrame?
OptFrame is an optimization framework focused in metaheuristic techniques, developed over 15 years by Brazilian research groups.

The OptFrame project has been hosted in SourceForge for many years, then moved to GitHub few years ago. It is open-source and dual licensed under LGPLv3 and MIT License (after version 4.3). The project [OptFrame](github.com/optframe/optframe) is originally written in C++.

### Why provide a Rust package for OptFrame?

OptFrame was born around 2008, before C++11 standard arrived. 
Coding in C++ at that time was amazing, but quite challenging. 

When C++11 arrived with move semantics and managed pointers (unique_ptr and shared_ptr), OptFrame started using those features, what helped a lot to safely manage memory, without big impacts on performance.

When C++14 and C++17 arrived, many other nice stuff was adopted, including optionals and tuples.

When C++20 arrived, concepts was heavily introduced in OptFrame, what helped managing templates and also experimentation with coroutines.

#### So, why try Rust now?

I've been a fan of novel languages for a while, and Rust is certainly attracting interesting developers. It is becoming more and more challenging to find new developers in C++, so I've been creating this feeling of *trying Rust* for a while...

During 2021 and 2022, some nice students have demonstrated interest in trying OptFrame, but only if it was written in Rust... Yesterday (19/01/2022), a profound chat with a nice student finally convinced me that it's time to try Rust. I thought that, if Rust is truly a nice language, it wouldn't take more than 24 hours for experienced C++ developers to port a tiny (but central) part of OptFrame and experiment it here (full project will certainly take months). So here we are! At 20/01/2022, we are proud to announce a first sketch of OptFrame in Rust :)

#### First Impressions of Rust

The ideas of modern C++ and Rust regarding memory management are not so different, so when you think of Box as unique_ptr, few things needed to be changed. Hopefully, C++ `concepts` and `classes`, compared to Rust `traits` are also quite similar, for our use case.

C++ still misses nice modularization. The advantage of Rust is quite clear regarding *shorter and cleaner* error messages, so as providing a *standard package manager* and *standard build system* (cargo).

## Try OptFrame in Rust

I guess it's just `cargo run`.

See `main.rs` for a draft example, which follows the same logic as OptFrame TSP Example in [Quickstart](https://optframe.readthedocs.io/en/latest/quickstart.html).

### Versioning

This project will start from alpha versions (0.x) and then follow OptFrame C++ major version,
starting from 5.x.
After that, typical semantic versioning may apply (for features and bugs).
However, Major version is expected to be in sync with OptFrame Project C++.

## Acknowledgements

I appreciate all advices from friends and students, specially Eduardo and Victor, for motivating me to create this OptFrame port as quickly as possible.

## License

Dual LGPLv3 and MIT License

Copyleft 2022

Igor Machado Coelho