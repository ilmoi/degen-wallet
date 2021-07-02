const ethers = require('ethers')

const provider = new ethers.providers.JsonRpcProvider("http://localhost:7545")
// console.log(provider)
const signer = provider.getSigner("0xCd550E94040cEC1b33589eB99B0E1241Baa75D19")
// console.log(signer)

async function play() {
  // console.log(await provider.getBlockNumber())
  // console.log(await provider.getBalance("0xCd550E94040cEC1b33589eB99B0E1241Baa75D19"))

  // const tx = signer.sendTransaction({to: "0xC48ad5fd060e1400a41bcf51db755251AD5A2475", value: ethers.utils.parseEther("0.1")})

  console.log(ethers.utils.defaultPath)
  console.log()

}

play()
