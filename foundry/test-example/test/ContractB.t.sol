pragma solidity ^0.8.13;

import "forge-std/Test.sol";

contract ContractBTest is Test {
    uint256 testNumber;

    function setUp() public {
        testNumber = 42;
    }

    function test_NumberIs42() public {
        assertEq(testNumber, 42);
    }

    function testFail_Subtract43() public {
        testNumber -= 43;
    }

    function test_CannotSubtract43() public {
        vm.expectRevert(stdError.arithmeticError);
        testNumber -= 43;
    }
}

abstract contract HelperContract {
    uint256 testNumber;
}

contract MyContractTest is Test, HelperContract {
    function setUp() public {
        testNumber = 42;
    }

    function test_NumberIs42() public {
        assertEq(testNumber, 42);
    }
}

contract MyOtherContractTest is Test, HelperContract {
    function setUp() public {
        testNumber = 43;
    }

    function test_NumberIs43() public {
        assertEq(testNumber, 43);
    }
}
