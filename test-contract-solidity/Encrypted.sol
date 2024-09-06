// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IDecrypter {
    function decrypt(uint8[] memory c, uint8[] memory skbytes) external returns (uint8[] memory);
}

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
