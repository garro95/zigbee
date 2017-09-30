# zigbee
A crate for the Rust programming Laguage that aims to provide a common interface for the interoperability of other crates that implements parts of the ZigBee stack.

## Introduction to ZigBee
ZigBee is a standard for the implementation of a "low-power, low-cost, low-complexity networking for the Internet of Things".

It is based on the IEEE 802.15.4 specification and can be used to create Personal Area Networks with small, low power digital radios for home automation, medical device data collection, and other low-power low-bandwidth needs, designed for small scale projects which need wireless connection. Hence, Zigbee is a low-power, low data rate, and close proximity (i.e., personal area) wireless ad hoc network. For more information see [wikipedia](https://en.wikipedia.org/wiki/Zigbee) and the [ZigBee Alliance website](http://www.zigbee.org/).

## The `zigbee` crate
The crate aims to provide a common interface made of `trait`s, `enum`s, `struct`s and `const`s for the  interoperability of other crates that implements parts of the standard, such as implementations of some layer of the stack or device driver for hardware that implements some layers internally (e.g. a USB dongle like [Texas Instruments cc2531](http://www.ti.com/tool/cc2531emk)).

## Request for help
The ZigBee standard is very wide and the specification is a document of more then 600 pages.

By now, just the Application Support sub-layer was implemented and the interface presents some critical points I would like to address.

Help provided with Issues and Pull Request will be appreciated.

## License
This crate is doubly licensed under the Apache and MIT licenses.
