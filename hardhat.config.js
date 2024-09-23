require("@nomiclabs/hardhat-ethers");

module.exports = {
  solidity: "0.8.10",
  networks: {
    sepolia: {
      url: "https://sepolia.infura.io/v3/YOUR-PROJECT-ID",
      accounts: ["YOUR-PRIVATE-KEY"]
    }
  }
};