// SPDX-License-Identifier: MIT
pragma solidity ^0.8.10;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "./interfaces/IFlashLoanProvider.sol";
import "./interfaces/IArbitrageStrategy.sol";

contract FlashLoanArbitrage is Ownable, ReentrancyGuard {
    IFlashLoanProvider public flashLoanProvider;
    IArbitrageStrategy public arbitrageStrategy;
    
    uint256 public minProfitThreshold;
    uint256 public maxFlashLoanAmount;
    
    event ArbitrageExecuted(address indexed asset, uint256 amount, uint256 profit);
    event StrategyUpdated(address indexed newStrategy);
    event FlashLoanProviderUpdated(address indexed newProvider);
    
    constructor(address _flashLoanProvider, address _arbitrageStrategy) {
        flashLoanProvider = IFlashLoanProvider(_flashLoanProvider);
        arbitrageStrategy = IArbitrageStrategy(_arbitrageStrategy);
        minProfitThreshold = 0.1 ether; // Default profit threshold
        maxFlashLoanAmount = 1000 ether; // Default max flash loan amount
    }
    
    function executeArbitrage(address[] calldata assets, uint256[] calldata amounts, bytes calldata params) external onlyOwner nonReentrant {
        require(assets.length == amounts.length, "Mismatched assets and amounts");
        require(amounts[0] <= maxFlashLoanAmount, "Exceeds max flash loan amount");
        
        flashLoanProvider.flashLoan(address(this), assets[0], amounts[0], params);
    }
    
    function onFlashLoan(address initiator, address asset, uint256 amount, uint256 fee, bytes calldata params) external returns (bytes32) {
        require(msg.sender == address(flashLoanProvider), "Unauthorized flash loan provider");
        require(initiator == address(this), "Unauthorized initiator");
        
        uint256 profit = arbitrageStrategy.executeArbitrage(asset, amount, params);
        require(profit > fee + minProfitThreshold, "Insufficient profit");
        
        // Repay flash loan
        IERC20(asset).transfer(address(flashLoanProvider), amount + fee);
        
        emit ArbitrageExecuted(asset, amount, profit - fee);
        
        return keccak256("ERC3156FlashBorrower.onFlashLoan");
    }
    
    function updateStrategy(address _newStrategy) external onlyOwner {
        arbitrageStrategy = IArbitrageStrategy(_newStrategy);
        emit StrategyUpdated(_newStrategy);
    }
    
    function updateFlashLoanProvider(address _newProvider) external onlyOwner {
        flashLoanProvider = IFlashLoanProvider(_newProvider);
        emit FlashLoanProviderUpdated(_newProvider);
    }
    
    function setMinProfitThreshold(uint256 _threshold) external onlyOwner {
        minProfitThreshold = _threshold;
    }
    
    function setMaxFlashLoanAmount(uint256 _maxAmount) external onlyOwner {
        maxFlashLoanAmount = _maxAmount;
    }
    
    // Emergency functions
    function emergencyWithdraw(address token) external onlyOwner {
        uint256 balance = IERC20(token).balanceOf(address(this));
        IERC20(token).transfer(owner(), balance);
    }
}
