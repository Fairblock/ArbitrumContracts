// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IDecrypter {
    function decrypt(uint8[] memory c, uint8[] memory skbytes) external returns (uint8[] memory);
}

/**
 * @title Simple Confidentiality App Example
 * @author Fairblock Technologies
 * @notice Example contract showing simple flow for a Confidentiality App using Fairblock Technologies.
 * @dev FLOW OF CONTRACT:
 *  Deploy Contract: Deploys MessageStorage on the Arbitrum Sepolia testnet.
 * Submit Encrypted Message for storage: Sends an encrypted message to the contract. It populates a mapping of conditions to arrays of EncryptedMessage structs. This is key to confidential apps, as Fairyring provides decryption keys to use to decrypt these encrypted messages, they need to be stored somewhere in wait.
 * Decrypt by Submitting Key & Condition: Ultimately calls `decrypt()` on `Decrypter` contract. Does this by sending a key to the decrypter contract, that it has obtained by listening to fairyring, and the corresponding ciphertexts (encrypted msgs) to be decrypted. The condition is used to get the encrypted message wrt to the condition. It then gets element-specific decrypted messages, and populates the mapping `messagesByCondition` with them. 
 * Read and Decode: Retrieves the encrypted message from the contract, decodes it, and prints the original message. 
 */
contract MessageStorage {
    struct EncryptedMessage {
        address sender;
        uint8[] ciphertext;
        string condition;
        bool isDecrypted;
        uint8[] plaintext;
    }

    
    mapping(string => EncryptedMessage[]) public messagesByCondition;

    address public decrypter;

    constructor(address _decrypter) {
        decrypter = _decrypter;
    }

    /**
     * @notice Submit array of encrypted messages and condition to be encrypted
     * @param ciphertext array of encrypted messages
     * @param condition used to organize encrypted messages within contract storage
     */
    function submitMessage(uint8[] memory ciphertext, string memory condition) public {
        EncryptedMessage memory newMessage = EncryptedMessage({
            sender: msg.sender,
            ciphertext: ciphertext,
            condition: condition,
            isDecrypted: false,
            plaintext: new uint8[](0)
        });

        messagesByCondition[condition].push(newMessage);

      
    }

    /**
     * @notice Sends a key (in the form of a byte array) to the contract. Part of the decryption mechanism in contract.
     * @param condition at which the decryption process can take place
     * @param key obtained from listening to fairyring
     * @dev This would typically be done via a Keeper (Chainlink, etc.) such that it triggers automatically once the decryption key is obtained from Fairyring. 
     * @dev instead of simply storing the decrypted messages, this is when more interesting functionality can be carried out. Ex.) Carrying out an actual swap execution at the beginning of the block.
     */
    function submitKey(string memory condition, uint8[] memory key) public {
        require(messagesByCondition[condition].length > 0, "No messages for this condition");

        EncryptedMessage[] storage messages = messagesByCondition[condition];

        for (uint i = 0; i < messages.length; i++) {
            if (!messages[i].isDecrypted) {
              
               uint8[] memory out = IDecrypter(decrypter).decrypt(messages[i].ciphertext, key);
                messages[i].isDecrypted = true;
                messages[i].plaintext = out;

              
            }
        }
    }

  
    /**
     * @notice Returns decrypted messages corresponding to a condition.
     * @param condition specified as the trigger point for decrypting respective encrypted messages
     */
    function checkMessages(string memory condition) public view returns (uint8[][] memory){
       
        EncryptedMessage[] storage messages = messagesByCondition[condition];
         uint8[][] memory messageList = new uint8[][](messages.length);
        uint j = 0;
        for(uint i = 0; i < messages.length; i++){
        if (messages[i].isDecrypted) {
            messageList[j] = (messages[i].plaintext);
            j++;
        }
        }
        return messageList;
    }
}
