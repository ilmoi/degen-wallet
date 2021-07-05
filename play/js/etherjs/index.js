const ethers = require('ethers')

//https://docs.ethers.io/v5/api/utils/hdnode/

const provider = new ethers.providers.JsonRpcProvider("http://localhost:7545")
// console.log(provider)
const signer = provider.getSigner("0xCd550E94040cEC1b33589eB99B0E1241Baa75D19")
// console.log(signer)

async function play() {
  // console.log(await provider.getBlockNumber())
  // console.log(await provider.getBalance("0xCd550E94040cEC1b33589eB99B0E1241Baa75D19"))

  // const tx = signer.sendTransaction({to: "0xC48ad5fd060e1400a41bcf51db755251AD5A2475", value: ethers.utils.parseEther("0.1")})

  console.log(ethers.utils.defaultPath)

  const mnemonic = "earth life tennis bacon gorilla virus clip online clerk legal ice payment"

  //1 CREATE ROOT NODE
  //ok so this one line does all the 9 steps from the ethereum book
  const hdNode = ethers.utils.HDNode.fromMnemonic(mnemonic)
  console.log(hdNode)
  // console.log(hdNode.neuter()) //w/o private key

  //2 CREATE AS MANY DERIVED NODES AS YOU WANT
  const childNode = hdNode.derivePath("m/44'/60'/0/3/1/2/3")
  console.log(childNode)

  const seed = ethers.utils.mnemonicToSeed(mnemonic)
  console.log(seed)

  const entropy = ethers.utils.mnemonicToEntropy(mnemonic)
  console.log(entropy)
}

play()
