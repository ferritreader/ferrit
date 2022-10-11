# Ferrit

**Ferrit** is a front-end for Reddit, written in [Rust](https://www.rust-lang.org/). It is a fork of the [Libreddit project](https://github.com/spikecodes/libreddit) started by [spikecodes](https://spike.codes).

## Why Ferrit?

Ferrit is a fast, light, private, and secure way to browse Reddit. View the hottest takes on **/r/unpopularopinion** without having to enable Javascript or unwillingly submit any information to Reddit's servers. For more information, see the following pages on our [wiki](https://github.com/libbacon/ferrit/wiki):

* [FAQ/What are the advantages to using Ferrit over Reddit?](https://github.com/libbacon/ferrit/wiki/FAQ#what-are-the-advantages-to-using-ferrit-over-reddit): what Ferrit does and why it is preferable to visiting Reddit directly
* [FAQ/What are the disadvantages to using Ferrit over Reddit?](https://github.com/libbacon/ferrit/wiki/FAQ#what-are-the-disadvantages-to-using-ferrit-over-reddit): what Ferrit is _not_
* [Privacy/Reddit](https://github.com/libbacon/ferrit/wiki/Privacy#reddit): an acounting of the data Reddit may and will collect from its visitors

The [whole FAQ](https://github.com/libbacon/ferrit/wiki/FAQ) may address other questions you may have about Ferrit.

## Instances

Visit the [Instances wiki page](https://github.com/libbacon/ferrit/wiki/Instances) for a list of running Ferrit instances. _Note that this page is currently empty since there are no (known) instances of Ferrit that are in operation._

## Getting and Installing Ferrit

Visit [Building, Installation, and Deployment](https://github.com/libbacon/ferrit/wiki/Building,-Installation,-and-Deployment) on the wiki for detailed instructions on how to obtain and set up Ferrit.

## Comparison
### Speed

Last tested on Oct 11, 2022.

Results from [yellowlab tools](https://yellowlab.tools/).
|                        | [Libreddit](https://yellowlab.tools/result/gediwc3w6o)|[Reddit](https://yellowlab.tools/result/gediwsbyi4)|
|------------------------|---------------|------------|
| Requests               | 58            | 175        |
| Resource Size (card ui)| 4.53 MB       | 9.37 MB    |
| JS execution time      | **61 ms**     | **1915 ms** |
