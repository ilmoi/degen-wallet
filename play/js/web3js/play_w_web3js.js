const Web3 = require('web3')

const web3 = new Web3()

// https://www.youtube.com/watch?v=xexCCBbnLQc

// creates an address + private key
let acc = web3.eth.accounts.create()
let pk = acc.privateKey
console.log(acc)

// creates a keystore file by using a pw (the whole password stretching thing)
// you could actually store this keystore file on a remote server
let keystore = web3.eth.accounts.encrypt(pk, '123123')
console.log(keystore)

// test decryption - outputs exactly same as acc
let decrypted_acc = web3.eth.accounts.decrypt(keystore, '123123')
console.log(decrypted_acc)

// -----------------------------------------------------------------------------

// create wallet
let wallet = web3.eth.accounts.wallet.create(5) //with 5 accounts

let encrypted_wallet = wallet.encrypt('123123')
let decrypted_wallet = wallet.decrypt(encrypted_wallet, '123123')
console.log(decrypted_wallet)