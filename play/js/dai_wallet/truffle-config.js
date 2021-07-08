const HDWalletProvider = require("truffle-hdwallet-provider");
const mnemonic = "window recall kid brief dragon worry intact board thumb aunt hair cement";

require('babel-register');
require('babel-polyfill');

module.exports = {
  networks: {
    development: {
      host: "127.0.0.1",
      port: 7545,
      network_id: "*" // Match any network id
    },
    rinkeby: {
      provider: function () {
        return new HDWalletProvider(mnemonic, "https://rinkeby.infura.io/v3/ce0c2c5a809d408892888f67e83bf5e4");
      },
      network_id: 4,
      gas: 4500000,
      gasPrice: 10000000000,
    }
  },
  contracts_directory: './src/contracts/',
  contracts_build_directory: './src/abis/',
  compilers: {
    solc: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  }
}
