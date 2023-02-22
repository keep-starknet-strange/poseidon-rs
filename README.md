<div align="center">
  <h1>Poseidon-rs</h1>
  <img src="docs/images/poseidon_rs_img.png" height="200">
  <br />
  <a href="https://github.com/keep-starknet-strange/poseidon-rs/issues/new?assignees=&labels=bug&template=01_BUG_REPORT.md&title=bug%3A+">Report a Bug</a>
  -
  <a href="https://github.com/keep-starknet-strange/poseidon-rs/issues/new?assignees=&labels=enhancement&template=02_FEATURE_REQUEST.md&title=feat%3A+">Request a Feature</a>
  -
  <a href="https://github.com/keep-starknet-strange/poseidon-rs/discussions">Ask a Question</a>
</div>

<div align="center">
<br />

[![GitHub Workflow Status](https://github.com/keep-starknet-strange/poseidon-rs/actions/workflows/test.yml/badge.svg)](https://github.com/keep-starknet-strange/poseidon-rs/actions/workflows/test.yml)
[![Project license](https://img.shields.io/github/license/keep-starknet-strange/poseidon-rs.svg?style=flat-square)](LICENSE)
[![Pull Requests welcome](https://img.shields.io/badge/PRs-welcome-ff69b4.svg?style=flat-square)](https://github.com/keep-starknet-strange/poseidon-rs/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)

</div>

![](docs/images/poseidon-rs.gif)

<details>
<summary>Table of Contents</summary>

- [About](#about)
- [Warning](#warning)
- [Reference](#reference)
- [Implementation Design](#implementation-design)
  - [Poseidon Hash Function Overview](#poseidon-hash-function-overview)
  - [Poseidon Permutation](#poseidon-permutation)
  - [Round Function](#round-function)
  - [Constants Selection](#constants-selection)  
  - [Parameters](#parameters)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Build](#build)
  - [Test](#test)
- [Roadmap](#roadmap)
- [Support](#support)
- [Project assistance](#project-assistance)
- [Contributing](#contributing)
- [Authors \& contributors](#authors--contributors)
- [Security](#security)
- [License](#license)
- [Acknowledgements](#acknowledgements)

</details>

## About

>**Poseidon_rs** is an implementation in Rust of the Poseidon family of hash function.

It is being developed in the context of the [EIP 5988](https://eips.ethereum.org/EIPS/eip-5988) which proposes to introduce a new precompiled contract implementing Poseidon hash function over finite fields in order to provide an improved interoperability between the EVM and ZK & Validity rollups.  

## Warning

It is a work in progress, do not use in production.

## Reference

- EIP 5988: https://eips.ethereum.org/EIPS/eip-5988
- EIP 5988 discussion: https://ethereum-magicians.org/t/eip-5988-add-poseidon-hash-function-precompile/11772
- Poseidon paper: https://eips.ethereum.org/assets/eip-5988/papers/poseidon_paper.pdf
- Reference implementation: https://extgit.iaik.tugraz.at/krypto/hadeshash/-/tree/master/code

## Implementation Design

### Poseidon Hash Function Overview

This section describes how a hash of a message $M$ is computed using Poseidon.

Poseidon uses the sponge/squeeze technique to hash a message with an arbitrary size into a fixed-size output (see [Fig1](#figure-1-global-overview-of-a-poseidon-hash)).

The sponge has a state $S = (S_{1}, …, S_{t})$ made of $t$ field elements, initialized to zero. The state is divided into the outer state and the inner state, made of $r$ (rate) and $c$ (capacity) elements respectively.

<div align="center">
 
  #### **Figure 1. Global overview of a Poseidon hash**
  <img src="docs/images/rm-poseidon-fig-1.svg">
</div>

A sponge supports 2 operations: it can either absorb field elements or squeeze elements out.

To absorb a message of $r$ elements, the sponge adds the message to its outer state and applies the poseidon-permutation, leaving the sponge in a new state (see [Fig2](#figure-2-absorption)). More elements can be absorbed at will.

To squeeze elements out, the sponge returns all or a part of its outer state and applies the Poseidon-permutation to its state.

To hash a message, we first absorb it entirely and then squeeze the required number of elements out.

>The inner state is opaque: inputs don't directly modify it and it is never part of outputs. It is essential for security.

<div align="center">

  #### **Figure 2. Absorption**
  <img src="docs/images/rm-poseidon-fig-2.svg">
</div>


>Our implementation hash messages of length a multiple of the rate only. Supporting variable-length messages would require a sponge-compliant padding rule. 


### Poseidon Permutation 

A Poseidon permutation behaves like a random permutation. To achieve this, it applies many rounds of simpler permutations. Rounds come in 2 flavours: the more secure full rounds and the more efficient partial rounds (see [Fig3](#figure-3-poseidon-permutation)).

<div align="center">

  #### **Figure 3. Poseidon permutation**
  <img src="docs/images/rm-poseidon-fig-3.svg">
</div>

### Round Function

A round function consists of 3 transformations that modify the state:
- Ark: the round constants are added to the state.
- S-box: a substitution box, $Sbox(x)=x^α$, is applied with α chosen such that $gcd⁡(α,p-1)=1$.
- Mix: the state is mixed through a multiplication by a $t×t$ [MDS matrix](https://en.wikipedia.org/wiki/MDS_matrix).

In a full round function, S-boxes are applied to the full state while a partial round function contains a single S-box. Detailed overviews of both functions are given in [Fig4](#figure-5-partial-round-overview) and [Fig5](#figure-5-partial-round-overview).

<div align="center">

  #### **Figure 4. Full round overiew**
  <img src="docs/images/rm-poseidon-fig-4.svg">

  #### **Figure 5. Partial round overview**
  <img src="docs/images/rm-poseidon-fig-5.svg">

</div>

### Constants Selection

Hash's security depends on the selection of adequate round constants and MDS matrix. In turn, these constants depend on the finite field, the number of rounds, the rate and the capacity. This makes the Poseidon hash function family flexible, but one has to manage the parameters in some way.

Several propositions were made to overcome this difficulty in the context of EIP-5988, [among them those proposed by vbuterin](https://ethereum-magicians.org/t/eip-5988-add-poseidon-hash-function-precompile/11772) :

> 1. Add an extra global execution context variable which stores which state sizes and round counts have been used before, and the MDS and RC values for those state sizes using some standard algorithm. When using a new (state size, round count), generate new values and charge extra gas for this.
> 2. Generate parameters in real time.
> 3. Pass parameters in as inputs.

As a first step, we have chosen the first approach. Different set of parameters including MDS matrix and the round constants are hard coded in the library, but one could extend it with other sets of parameters.

### Parameters 

Parameters either pertains to the sponge construct or the permutation:

- Sponge:
    - The rate and the capacity. 
- Permutation:
    - The finite field, completely specified by its size. 
    - The number of full and partial rounds. 
    - The round constants and the MDS matrix. 

The following set of parameters are included in the library thus far: 

- Starkware: https://github.com/starkware-industries/poseidon 
- Mina: https://github.com/o1-labs/proof-systems/blob/ebe59f35f5cb6bb33fc0ed3c4cb5040d8cd81247/book/src/specs/poseidon.md 

## Getting Started

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install)

### Build

To build poseidon from source:

```bash
cargo build --release
```

#### Variants
You can add predefined poseidon variants with the features flag, for example:

```bash
cargo build --release --features starkware
```

The variants dependency tree is like so:

	| starkware
	|-- sw2
	|-- sw3
	|-- sw4
	|-- sw8
	| mina
	|-- pallas
	|-- vesta

This means that selecting feature starkware enables all four subvariants: sw2, sw3, sw4 and sw8.

#### std feature
By default, the library will compile as no_std. In fact, it compiles at the core layer, without even alloc.
However, no_std is omitted if the feature std is enabled.

#### c_bind feature
The library can include C-bindings for hash functions through the c_bind feature.
In that case, one would want to build a staticlib to link it into another program, using for example:

```bash
cargo rustc --crate-type staticlib --release --features c_bind,starkware
```

### Tests

To test the rust library:
```bash
cargo test
```

Tests for the C-interface are also available through golang in the c-bind-tests folder.
From that folder, one can run tests like so:

```bash 
go test -v
```
Note that golang must be installed on your system to run the go_tests.

Finally, we can also test that the library compiles with no_std compatibility in the ensure-no-std folder.
From there, we can compile the test package like so:

```bash
cargo rustc --target thumbv7em-none-eabihf
```

The test passes if it compiles.
Note that you can use another no_std target if you wish, and need to have it installed in your environment.


## Roadmap

See the [open issues](https://github.com/keep-starknet-strange/poseidon-rs/issues) for
a list of proposed features (and known issues).

- [Top Feature Requests](https://github.com/keep-starknet-strange/poseidon-rs/issues?q=label%3Aenhancement+is%3Aopen+sort%3Areactions-%2B1-desc)
  (Add your votes using the 👍 reaction)
- [Top Bugs](https://github.com/keep-starknet-strange/poseidon-rs/issues?q=is%3Aissue+is%3Aopen+label%3Abug+sort%3Areactions-%2B1-desc)
  (Add your votes using the 👍 reaction)
- [Newest Bugs](https://github.com/keep-starknet-strange/poseidon-rs/issues?q=is%3Aopen+is%3Aissue+label%3Abug)

## Support

Reach out to the maintainer at one of the following places:

- [GitHub Discussions](https://github.com/keep-starknet-strange/poseidon-rs/discussions)
- Contact options listed on
  [this GitHub profile](https://github.com/starknet-exploration)

## Project assistance

If you want to say **thank you** or/and support active development of poseidon-rs:

- Add a [GitHub Star](https://github.com/keep-starknet-strange/poseidon-rs) to the
  project.
- Tweet about the poseidon-rs.
- Write interesting articles about the project on [Dev.to](https://dev.to/),
  [Medium](https://medium.com/) or your personal blog.

Together, we can make poseidon-rs **better**!

## Contributing

First off, thanks for taking the time to contribute! Contributions are what make
the open-source community such an amazing place to learn, inspire, and create.
Any contributions you make will benefit everybody else and are **greatly
appreciated**.

Please read [our contribution guidelines](docs/CONTRIBUTING.md), and thank you
for being involved!

## Authors & contributors

For a full list of all authors and contributors, see
[the contributors page](https://github.com/keep-starknet-strange/poseidon-rs/contributors).

## Security

poseidon-rs follows good practices of security, but 100% security cannot be assured.
poseidon-rs is provided **"as is"** without any **warranty**. Use at your own risk.

_For more information and to report security issues, please refer to our
[security documentation](docs/SECURITY.md)._

## License

This project is licensed under the **MIT license**.

See [LICENSE](LICENSE) for more information.

## Acknowledgements

This implementation is inspired by earlier rust implementations. We would like to thank [arnaudcube](https://github.com/arnaucube) and [neptune](https://github.com/filecoin-project/neptune)’s contributors. 
