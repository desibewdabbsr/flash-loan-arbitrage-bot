const { expect } = require("chai");
const sinon = require("sinon");
const BlockchainInterface = require("../src/blockchain_interface");

describe("BlockchainInterface", function () {
  let blockchainInterface;
  let providerStub;
  let contractStub;

  beforeEach(function () {
    providerStub = {
      getSigner: sinon.stub().returns({
        sendTransaction: sinon.stub().resolves({ wait: () => Promise.resolve({ status: 1 }) }),
      }),
      getGasPrice: sinon.stub().resolves(ethers.utils.parseUnits("50", "gwei")),
    };

    contractStub = {
      estimateGas: {
        executeArbitrage: sinon.stub().resolves(ethers.BigNumber.from("200000")),
      },
      executeArbitrage: sinon.stub().resolves({ wait: () => Promise.resolve({ status: 1 }) }),
    };

    blockchainInterface = new BlockchainInterface("http://localhost:8545", "0x1234567890123456789012345678901234567890");
    blockchainInterface.provider = providerStub;
    blockchainInterface.contract = contractStub;
  });

  it("Should execute arbitrage successfully", async function () {
    const assets = ["0x1111111111111111111111111111111111111111"];
    const amounts = [ethers.utils.parseEther("100")];
    const params = "0x";

    const result = await blockchainInterface.executeArbitrage(assets, amounts, params);
    expect(result.status).to.equal(1);
  });

  // Add more tests for other methods and edge cases
});
