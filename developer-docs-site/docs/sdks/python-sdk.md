---
title: "Python SDK"
slug: "python-sdk"
---

# Pont Python SDK

Pont provides a lightly maintained official Python SDK. It is available on [PyPi](https://pypi.org/project/pont-sdk/) with the source code in the [Pont-core GitHub repository](https://github.com/aptos-labs/pont-core/tree/main/ecosystem/python/sdk). Much of the functionality of the Python SDK mirrors the [Typescript SDK](/sdks/ts-sdk/index). The primary purpose of the Python SDK is to help Python developers to quickly become familiar with Pont and as an accompaniment to Pont tutorials.

## Installing Python SDK

The Python SDK can either be installed via `pip`, from source, or embedded:

### Install with pip

To install via `pip`:

```bash
pip3 install pont-sdk
```

The `pont-sdk` will be installed in the local site packages directory. For example, on macOS, you will find the `pont-sdk` in the `~/Library/Python/3.8/lib/python/site-packages/pont_sdk` directory.

### Install from the source

To install from source:

```bash
git clone https://github.com/aptos-labs/pont-core
cd pont-core/ecosystem/python/sdk
python3 setup.py install --user
```

### Install by embedding

To embed the Python SDK into your existing Python project:

```
cd /path/to/python/project
cp -r /path/to/pont-core/ecosystem/python/sdk/pont-sdk pont-sdk
```

## Using Python SDK

See the [Developer Tutorials](/tutorials/index.md) for code examples showing how to use the Python SDK.
