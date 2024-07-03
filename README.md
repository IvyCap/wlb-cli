<!-- PROJECT SHIELDS -->
[![Rust][rust-lang.org]][Rust-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h3 align="center">Work Life Balance CLI</h3>

  <p align="center">
    A Linux command line tool to track tasks/work and display the historic number of hours, and percentage of a specified timeframe that tasks/work take. 
  </p>
</div>


<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#prerequisites">Prerequisites</a></li>
    <li><a href="#installation">Installation</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>

## Prerequisites

The things you need before installing the software.

* A Linux Disto
* Rust
* git

## Installation

A step by step guide that will tell you how to get the development environment up and running.

```
$ git clone https://github.com/IvyCap/wlb-cli.git
$ cd wlb-cli
$ cargo build
```

## Usage

A few examples of useful commands and/or tasks.

```
$ wlb-cli
$ wlb-cli task --add
$ wlb-cli task --edit
$ wlb-cli review --ytd
$ wlb-cli review --month
```

## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[license-shield]: https://img.shields.io/github/license/othneildrew/Best-README-Template.svg?style=for-the-badge
[license-url]: https://github.com/IvyCap/wlb-cli/blob/main/LICENSE
[rust-lang.org]: https://img.shields.io/badge/Rust-ffffff?style=for-the-badge&logo=rust&logoColor=black
[Rust-url]: https://www.rust-lang.org