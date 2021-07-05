const Migrations = artifacts.require("Migrations");
const DaiTokenMock = artifacts.require("DaiTokenMock");

module.exports = async function(deployer) {
  await deployer.deploy(Migrations);
  await deployer.deploy(DaiTokenMock);

  const tokenMock = await DaiTokenMock.deployed();
  await tokenMock.mint(
    '0xCd550E94040cEC1b33589eB99B0E1241Baa75D19',
    '1000000000000000000000'
  )
};
