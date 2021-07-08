const Migrations = artifacts.require("Migrations");
const DaiTokenMock = artifacts.require("DaiTokenMock");

module.exports = async function(deployer, network, accounts) {
  await deployer.deploy(Migrations, {from: accounts[0]});
  await deployer.deploy(DaiTokenMock, {from: accounts[0]});

  const tokenMock = await DaiTokenMock.deployed();
  console.log(tokenMock);

  await tokenMock.mint(
    accounts[0],
    '1000000000000000000000',
    {from: accounts[0]}
  )
};
