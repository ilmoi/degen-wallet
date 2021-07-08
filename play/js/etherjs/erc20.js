const ethers = require('ethers')

//https://docs.ethers.io/v5/api/utils/hdnode/

const provider = new ethers.providers.getDefaultProvider("rinkeby");
// console.log(provider)
// const signer = provider.getSigner("0xCd550E94040cEC1b33589eB99B0E1241Baa75D19")
// console.log(signer)

let tokenAddress = "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984";
let walletAddress = "0xCd550E94040cEC1b33589eB99B0E1241Baa75D19";

let tokenAbi = [
  // Get the account balance
  "function balanceOf(address) view returns (uint)",
]

let token_contract = new ethers.Contract(tokenAddress, tokenAbi, provider);

async function play() {
  let b = await token_contract.balanceOf(walletAddress);
  b = ethers.utils.formatUnits(b, 18);
  console.log(b);
}

play()