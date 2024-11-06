// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IDecrypter {
    function decrypt(uint8[] memory c, uint8[] memory skbytes) external returns (uint8[] memory);
}

/**
 * @title Simple Sealed Bid Auction App Example
 * @notice Example Auction App showcasing Solidity and Arbitrum Stylus Integrations with Fairblock Technologies. 
 * @dev Functions as a sealed-bid auction where bids are submitted encrypted and revealed using a decryption key once a certain time is passed, triggering the end of the auction. The auctionOwner gets the bid amount; this is assuming that the auction is tied to some offchain deliverable (Art auction etc.).
 * @dev TODO: actual transference of funds are commented out while example is troubleshot
 */
contract SealedBidAuctionExample {

    /// @notice Represents a bid entry in the auction
    struct BidEntry {
        address bidder;        // Address of the bidder
        uint8[] encryptedBid;  // Encrypted bid amount
        bool isDecrypted;      // Whether the bid has been decrypted
        uint256 bidValue;      // The actual bid value after decryption
    }

    /// @notice List of all bids in the auction
    BidEntry[] public bids;

    /// @notice Owner of the auction who receives the highest bid amount
    address public auctionOwner;

    /// @notice Reference to an external decryption contract
    IDecrypter public decrypterContract;

    /// @notice Block number after which bids can be revealed
    uint256 public bidCondition;

    /// @notice Fee required to submit a bid
    uint256 public auctionFee;

    /// @notice The highest bid amount after the auction is finalized
    uint256 public highestBid;

    /// @notice The address of the highest bidder
    address public highestBidder;

    /// @notice Indicates if the auction has been finalized
    bool public auctionFinalized;

    /// @dev Event emitted when the auction is initialized
    /// @param deadline The block number after which bids can be revealed
    /// @param fee The fee required to participate in the auction
    event AuctionInitialized(uint256 deadline, uint256 fee);

    /// @dev Event emitted when a new bid is submitted
    /// @param bidder Address of the bidder
    /// @param bidIndex Index of the bid in the bids array
    event BidSubmitted(address bidder, uint256 bidIndex);

    /// @dev Event emitted when the auction is finalized
    /// @param winner Address of the winning bidder
    /// @param winningBid The highest bid amount
    event AuctionFinalized(address winner, uint256 winningBid);

    /// @dev Event emitted when a refund is issued to a non-winning bidder
    /// @param bidder Address of the bidder receiving the refund
    /// @param amount Amount refunded
    event RefundIssued(address bidder, uint256 amount);

    /**
     * @notice Initializes the auction with a decryption contract, a deadline, and a fee.
     * @param _decrypter Address of the decryption contract
     * @param _deadline The block number after which bids can be revealed
     * @param _fee The fee required to submit a bid
     */
    constructor(address _decrypter, uint256 _deadline, uint256 _fee) {
        auctionOwner = msg.sender;
        decrypterContract = IDecrypter(_decrypter);
        bidCondition = _deadline;
        auctionFee = _fee;
        auctionFinalized = false;
        emit AuctionInitialized(_deadline, _fee);
    }

    /**
     * @notice Submits an encrypted bid along with the required fee.
     * @param encryptedBid The encrypted bid value in `uint8[]` format
     * @dev TODO: this function would be called by different addresses, likely as per a tx flow where, within the same tx, a series of functions would be called where: 1. Bid made, 2. Bid encrypted, 3. Bid submitted here. For test purposes, we keep the bidder to the one test address. 
     */
    function submitEncryptedBid(uint8[] calldata encryptedBid) 
        external 
        payable 
    {
        require(block.timestamp > bidCondition, "Auction deadline passed");
        // require(msg.value >= auctionFee, "Insufficient fee");

        bids.push(BidEntry({
            bidder: msg.sender,
            encryptedBid: encryptedBid,
            isDecrypted: false, 
            bidValue: 0
        }));

        emit BidSubmitted(msg.sender, bids.length - 1);
    }

    /**
     * @notice Reveals all bids using the provided decryption key and determines the winner.
     * @param decryptionKey The decryption key to unlock the encrypted bids
     * @dev TODO: edit helper `uint8ArrayToUint256` such that it can work with a salt being part of the decrypted message. AKA - (bid, salt) --> we need it to get the bid from the decrypted message; this would mean working with say 4 uint8 elements at the end of each encrypted message such that those 4 uint8 elements were the salt. EX.) uint8[] bid_salt = [1, 2, 3, 4, 0, 0, 0, 0] ; where the first 4 elements are the bids.
     */
    function revealBids(uint8[] calldata decryptionKey) external {
        require(block.timestamp >= bidCondition, "Auction still ongoing");
        require(!auctionFinalized, "Auction already finalized");

        uint256 highestBidLocal = 0;
        address highestBidderLocal = address(0);

        for (uint256 i = 0; i < bids.length; i++) {
            uint8[] memory out = decrypterContract.decrypt(
                bids[i].encryptedBid,
                decryptionKey
            ); // TODO: as a user, I can blindly copy someone else's bid by copying the c.
            bids[i].isDecrypted = true;
            uint256 bidValue = uint8ArrayToUint256(out);
            if (bidValue > highestBidLocal) {
                highestBidLocal = bidValue;
                highestBidderLocal = bids[i].bidder;
            }
            bids[i].bidValue = bidValue;
        }

        highestBid = highestBidLocal;
        highestBidder = highestBidderLocal;
        auctionFinalized = true;

        // payable(auctionOwner).transfer(highestBid);

        emit AuctionFinalized(highestBidder, highestBid);
    }

    /**
     * @notice Issues refunds to all non-winning bidders after the auction is finalized.
     */
    function issueRefunds() external {
        require(auctionFinalized, "Auction not finalized");

        for (uint256 i = 0; i < bids.length; i++) {
            if (bids[i].bidder != highestBidder) {
                uint256 refundAmount = bids[i].bidValue - auctionFee;
                // payable(bids[i].bidder).transfer(refundAmount);
                emit RefundIssued(bids[i].bidder, refundAmount);
            }
        }
    }

    /**
     * @dev Utility function to convert a `uint8[]` array to a `uint256`.
     * @param arr The input `uint8[]` array
     * @return result The resulting `uint256` value
     */
   function uint8ArrayToUint256(uint8[] memory arr) public pure returns (uint) {
        uint result = 0;
        for (uint i = 0; i < arr.length; i++) {
            require(arr[i] >= 48 && arr[i] <= 57, "Array contains non-numeric characters");
            result = result * 10 + (arr[i] - 48);
        }
        return result;
    }
}
