#!/usr/bin/env node

import { HDNodeWallet, getAddress, version as ethersVersion } from "ethers";
import process from "node:process";

const DEFAULT_PATH = "m/44'/60'/0'/0/0";

function usage() {
  return `Cross-check an Ethereum address from a mnemonic with ethers.js.

Usage:
  node scripts/cross-check-address.mjs --mnemonic "<words>" [--path "<path>"] [--expected "<address>"] [--show-mnemonic]
  printf '%s\n' '<words>' | node scripts/cross-check-address.mjs --stdin [--path "<path>"] [--expected "<address>"] [--show-mnemonic]

Options:
  --mnemonic <words>     Read the mnemonic from a command argument.
  --stdin                Read the mnemonic from standard input.
  --path <path>          Derivation path. Default: ${DEFAULT_PATH}
  --passphrase <text>    Optional BIP39 passphrase. Default: empty string.
  --expected <address>   Expected address to compare against.
  --show-mnemonic        Echo the mnemonic back to stdout.
  -h, --help             Show this help.
`;
}

function parseArgs(argv) {
  const options = {
    expected: null,
    mnemonic: null,
    passphrase: "",
    path: DEFAULT_PATH,
    showMnemonic: false,
    stdin: false
  };

  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];

    switch (argument) {
      case "--mnemonic":
        options.mnemonic = valueFor(argv, index, argument);
        index += 1;
        break;
      case "--stdin":
        options.stdin = true;
        break;
      case "--path":
        options.path = valueFor(argv, index, argument);
        index += 1;
        break;
      case "--passphrase":
        options.passphrase = valueFor(argv, index, argument);
        index += 1;
        break;
      case "--expected":
        options.expected = valueFor(argv, index, argument);
        index += 1;
        break;
      case "--show-mnemonic":
        options.showMnemonic = true;
        break;
      case "-h":
      case "--help":
        console.log(usage());
        process.exit(0);
      default:
        throw new Error(`Unknown argument: ${argument}`);
    }
  }

  if (options.mnemonic !== null && options.stdin) {
    throw new Error("Use either --mnemonic or --stdin, not both.");
  }

  if (options.mnemonic === null && !options.stdin) {
    throw new Error("Provide a mnemonic with --mnemonic or pipe one in with --stdin.");
  }

  return options;
}

function valueFor(argv, index, flag) {
  const value = argv[index + 1];

  if (value === undefined) {
    throw new Error(`Missing value for ${flag}.`);
  }

  return value;
}

async function readMnemonic(options) {
  if (options.mnemonic !== null) {
    return normalizeMnemonic(options.mnemonic);
  }

  let input = "";

  for await (const chunk of process.stdin) {
    input += chunk;
  }

  if (input.trim() === "") {
    throw new Error("Stdin did not contain a mnemonic.");
  }

  return normalizeMnemonic(input);
}

function normalizeMnemonic(input) {
  return input
    .trim()
    .split(/\s+/u)
    .map((word) => word.toLowerCase())
    .join(" ");
}

function normalizeAddress(address) {
  return getAddress(address);
}

async function main() {
  try {
    const options = parseArgs(process.argv.slice(2));
    const mnemonic = await readMnemonic(options);
    const wallet = HDNodeWallet.fromPhrase(mnemonic, options.passphrase, options.path);
    const derivedAddress = normalizeAddress(wallet.address);

    console.log(`Cross-check implementation: ethers/${ethersVersion}`);
    if (options.showMnemonic) {
      console.log(`Mnemonic: ${mnemonic}`);
    }
    console.log(`Derivation path: ${options.path}`);
    console.log(`Derived address: ${derivedAddress}`);

    if (options.expected !== null) {
      const expectedAddress = normalizeAddress(options.expected);
      const matches = derivedAddress === expectedAddress;

      console.log(`Expected address: ${expectedAddress}`);
      console.log(`Match: ${matches ? "yes" : "no"}`);

      if (!matches) {
        process.exit(2);
      }
    }
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    console.error("");
    console.error(usage());
    process.exit(1);
  }
}

await main();
