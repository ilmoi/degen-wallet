const Web3 = require('web3')
const rpcURL = 'https://rinkeby.infura.io/v3/ce0c2c5a809d408892888f67e83bf5e4'
const web3 = new Web3(rpcURL)

let tokenAddress = "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984";
let walletAddress = "0xCd550E94040cEC1b33589eB99B0E1241Baa75D19";

// The minimum ABI to get ERC20 Token balance
let minABI = [
  // balanceOf
  {
    "constant":true,
    "inputs":[{"name":"_owner","type":"address"}],
    "name":"balanceOf",
    "outputs":[{"name":"balance","type":"uint256"}],
    "type":"function"
  },
  // decimals
  {
    "constant":true,
    "inputs":[],
    "name":"decimals",
    "outputs":[{"name":"","type":"uint8"}],
    "type":"function"
  }
];

let contract = new web3.eth.Contract(minABI,tokenAddress);

async function getBalance() {
  balance = await contract.methods.balanceOf(walletAddress).call();
  console.log(balance)
  return balance;
}

getBalance()