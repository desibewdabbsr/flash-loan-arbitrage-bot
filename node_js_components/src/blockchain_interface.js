const ethers = require('ethers');
const FlashLoanArbitrage = require('../artifacts/contracts/FlashLoanArbitrage.sol/FlashLoanArbitrage.json');

class BlockchainInterface {
    constructor(providerUrl, contractAddress) {
        this.provider = new ethers.providers.JsonRpcProvider(providerUrl);
        this.contract = new ethers.Contract(contractAddress, FlashLoanArbitrage.abi, this.provider);
        this.gasPrice = ethers.utils.parseUnits('50', 'gwei');  // Dynamic gas price strategy
    }

    async executeArbitrage(assets, amounts, params) {
        const signer = this.provider.getSigner();
        const gasLimit = await this.estimateGas(assets, amounts, params);
        const tx = await this.contract.connect(signer).executeArbitrage(assets, amounts, params, {
            gasLimit: gasLimit.mul(120).div(100),  // Add 20% buffer
            gasPrice: this.gasPrice
        });
        return await tx.wait();
    }

    async estimateGas(assets, amounts, params) {
        return await this.contract.estimateGas.executeArbitrage(assets, amounts, params);
    }

    async updateGasPrice() {
        const latestGasPrice = await this.provider.getGasPrice();
        this.gasPrice = latestGasPrice.mul(110).div(100);  // Set gas price to 110% of current price
    }
}

module.exports = BlockchainInterface;
