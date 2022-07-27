<div id="top"></div>
<!--
*** thanks for checking out the best-readme-template. if you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** don't forget to give the project a star!
*** thanks again! now go create something amazing! :d
-->



<!-- project shields -->
<!--
*** i'm using markdown "reference style" links for readability.
*** reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** see the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. this is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
<!-- [![contributors][contributors-shield]][contributors-url]
[![forks][forks-shield]][forks-url]
[![stargazers][stars-shield]][stars-url]
[![issues][issues-shield]][issues-url]
[![mit license][license-shield]][license-url]
[![linkedin][linkedin-shield]][linkedin-url] -->



<!-- project logo -->
<br />
<div align="center">
  <h2 align="center">reqwest.py</h2>

  <p align="center">
    a faster python http library
    <br />
    <a href="https://github.com/terabyte3/reqwest.py#getting-started"><strong>get started »</strong></a>
    <br />
    <br />
    <!-- <a href="https://github.com/terabyte3/reqwest.py">view benchmarks</a>
    · -->
    <a href="https://github.com/terabyte3/reqwest.py/issues">report a bug</a>
    ·
    <a href="https://github.com/terabyte3/reqwest.py/issues">request a feature</a>
  </p>
</div>



<!-- table of contents -->
<details>
  <summary>table of contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">about the project</a>
      <ul>
        <li><a href="#built-with">built with</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">getting started</a>
      <ul>
        <li><a href="#prerequisites">prerequisites</a></li>
        <li><a href="#installation">installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">usage</a></li>
    <li><a href="#roadmap">roadmap</a></li>
    <li><a href="#contributing">contributing</a></li>
    <li><a href="#license">license</a></li>
    <li><a href="#contact">contact</a></li>
    <li><a href="#acknowledgments">acknowledgments</a></li>
  </ol>
</details>



<!-- about the project -->
## about the project

![product name screen shot][product-screenshot]

there are so many great and fast http clients for python, but i feel that they could probably be faster. i'm trying to make a better http client for python, written in rust for extra speed and safety.


### built with
* ⚡ rust
* 🐍 python
* ✨ pyo3
* 🕸️ reqwest


<!-- getting started -->
## getting started

not much setup is required, since the compiled rust is in the wheel.
  ```sh
  $ pip install --user reqwest-py
  ```

<!-- usage examples -->
## usage
```py
>>> from reqwest import get
>>> get("https://example.com")
# or...
>>> from reqwest import Client
>>> Client().get("https://example.com")
```

<!-- roadmap -->
## roadmap

- [x] get requests
- [x] client
- [ ] crud support
- [x] headers
- [x] headers
- [ ] user agent
- [ ] caching that obeys cache-control headers

see the [open issues](https://github.com/terabyte3/reqwest.py/issues) for a full list of proposed features (and known issues).


<!-- contributing -->
## contributing

contributions are what make the open source community such an amazing place to learn, inspire, and create. any contributions you make are **greatly appreciated**.

if you have a suggestion that would make this better, please fork the repo and create a pull request. you can also simply open an issue with the tag "enhancement".
don't forget to give the project a star! thanks again!

1. fork the project
2. create your feature branch (`git checkout -b feature/amazingfeature`)
3. commit your changes (`git commit -m 'add some amazingfeature'`)
4. push to the branch (`git push origin feature/amazingfeature`)
5. open a pull request


<!-- license -->
## license

distributed under the mit license. see `license.txt` for more information.


<!-- contact -->
## contact

thrzl. - thrizzle@skiff.com

project link: [https://github.com/terabyte3/reqwest.py](https://github.com/terabyte3/reqwest.py)


<!-- acknowledgments -->
## acknowledgments

use this space to list resources you find helpful and would like to give credit to. i've included a few of my favorites to kick things off!

* [choose an open source license](https://choosealicense.com)
* [gitmoji](https://gitmoji.dev)


<!-- markdown links & images -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/terabyte3/reqwest.py.svg?style=for-the-badge
[contributors-url]: https://github.com/terabyte3/reqwest.py/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/terabyte3/reqwest.py.svg?style=for-the-badge
[forks-url]: https://github.com/terabyte3/reqwest.py/network/members
[stars-shield]: https://img.shields.io/github/stars/terabyte3/reqwest.py.svg?style=for-the-badge
[stars-url]: https://github.com/terabyte3/reqwest.py/stargazers
[issues-shield]: https://img.shields.io/github/issues/terabyte3/reqwest.py.svg?style=for-the-badge
[issues-url]: https://github.com/terabyte3/reqwest.py/issues
[license-shield]: https://img.shields.io/github/license/terabyte3/reqwest.py.svg?style=for-the-badge
[license-url]: https://github.com/terabyte3/reqwest.py/blob/master/license.txt
[linkedin-shield]: https://img.shields.io/badge/-linkedin-black.svg?style=for-the-badge&logo=linkedin&colorb=555
[linkedin-url]: https://linkedin.com/in/othneildrew
[product-screenshot]: assets/scrot.png
[next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logocolor=white
[next-url]: https://nextjs.org/
[react.js]: https://img.shields.io/badge/react-20232a?style=for-the-badge&logo=react&logocolor=61dafb
[react-url]: https://reactjs.org/
[vue.js]: https://img.shields.io/badge/vue.js-35495e?style=for-the-badge&logo=vuedotjs&logocolor=4fc08d
[vue-url]: https://vuejs.org/
[angular.io]: https://img.shields.io/badge/angular-dd0031?style=for-the-badge&logo=angular&logocolor=white
[angular-url]: https://angular.io/
[svelte.dev]: https://img.shields.io/badge/svelte-4a4a55?style=for-the-badge&logo=svelte&logocolor=ff3e00
[svelte-url]: https://svelte.dev/
[laravel.com]: https://img.shields.io/badge/laravel-ff2d20?style=for-the-badge&logo=laravel&logocolor=white
[laravel-url]: https://laravel.com
[bootstrap.com]: https://img.shields.io/badge/bootstrap-563d7c?style=for-the-badge&logo=bootstrap&logocolor=white
[bootstrap-url]: https://getbootstrap.com
[jquery.com]: https://img.shields.io/badge/jquery-0769ad?style=for-the-badge&logo=jquery&logocolor=white
[jquery-url]: https://jquery.com 