use ethers::core::{abi::AbiDecode, types::Bytes};
use ethers::prelude::{abigen, Abigen};
use ethers::providers::{Http, Provider};
use ethers::{prelude::*, types::U256, utils};
use std::convert::TryFrom;

//type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

abigen!(
    IERC20,
    r#"[
            function totalSupply() external view returns (uint256)
            function balanceOf(address account) external view returns (uint256)
            function transfer(address recipient, uint256 amount) external returns (bool)
            function allowance(address owner, address spender) external view returns (uint256)
            function approve(address spender, uint256 amount) external returns (bool)
            function transferFrom( address sender, address recipient, uint256 amount) external returns (bool)
            event Transfer(address indexed from, address indexed to, uint256 value)
            event Approval(address indexed owner, address indexed spender, uint256 value)
        ]"#,
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let erc20abi = [
        // Read-Only Functions
        "function balanceOf(address owner) view returns (uint256)",
        "function decimals() view returns (uint8)",
        "function symbol() view returns (string)",
        // Authenticated Functions
        "function transfer(address to, uint amount) returns (bool)",
        // Events
        "event Transfer(address indexed from, address indexed to, uint amount)",
    ];

    let provider =
        Provider::<Http>::try_from("https://goerli.infura.io/v3/af5be3175a8e43b2a5624cbce46e76b1")?;

    println!("Got provider {:?}", &provider);

    let contract_address = "0x02779305FCa5d9eb73ca159fc5088e904738897D";
    let from_account = "0xbd8efe701502b68ac670218b1f6a886287a7d197";
    let from_account_priv_key = "51a83e77a962ba5e049d789ee0b77713d71a371eecea57e0d9534ea720d87629";
    let to_account = "0xC05B52A6f22eB1EB1aE6Ed31F46D71f9Bf819D4d";
    //let _BINANCE_SMART_CHAIN_NETWORK_URL = "";

    let wallet: LocalWallet = from_account_priv_key
        .parse::<LocalWallet>()?
        .with_chain_id(Chain::Goerli);
    println!("Got Wallet {:?}", wallet);

    let signer = SignerMiddleware::new(provider.clone(), wallet.clone());
    println!("Got Signer {:?}", signer);

    let from_address = from_account.parse::<Address>()?;
    let from_address_balance_wei = provider.get_balance(from_address, None).await?;
    let balance_str = utils::format_ether(from_address_balance_wei);
    println!(
        "from_address {:?} from_address_balance_wei {:?} balance_str {:?}",
        from_address, from_address_balance_wei, balance_str
    );

    let contract_address = contract_address.parse::<Address>()?;
    let balance_contract = provider.get_balance(contract_address, None).await?;
    println!("balance_contract {:?}", balance_contract);

    //let code = provider.get_code(contract_address, None).await?;
    //println!("code {:?}", code);

    let erc20_contract_abi = ethers::abi::parse_abi(&erc20abi)?;
    let erc20_contract = ethers::contract::Contract::new(
        contract_address,
        erc20_contract_abi.clone(),
        provider.clone(),
    );

    let erc20_decimals: u32 = erc20_contract
        .method::<_, u32>("decimals", ())?
        .call()
        .await?;

    println!("decimals: {:?}", erc20_decimals);

    let erc20_balance: U256 = erc20_contract
        .method::<_, U256>("balanceOf", from_address)?
        .call()
        .await?;
    let erc20_balance_decoded = utils::format_units(erc20_balance, erc20_decimals)?;
    println!(
        "erc20_contract_address {:?} decimals {:?} erc20_balance {:?} erc20_balance_decoded {:?}",
        contract_address, erc20_decimals, erc20_balance, erc20_balance_decoded
    );

    let to_address = to_account.parse::<Address>()?;

    /*
    let tx = TransactionRequest::new()
        .to(to_address)
        .value(U256::from(utils::parse_ether(0.0001)?))
        .from(from_address);

    let tx = signer.send_transaction(tx, None).await?.await?;
    println!("Transaction Receipt: {:?}", &tx);
    */

    let erc20_rw_contract = ethers::contract::Contract::new(
        contract_address,
        erc20_contract_abi.clone(),
        signer.clone(),
    );

    let erc20_transfer_value = U256::from(utils::parse_units(1, erc20_decimals)?);
    println!("ERC20 formatted transfer value: {:?}", erc20_transfer_value);

    /*

    let method_call = erc20_rw_contract.method::<_, bool>(
        "transfer",
        vec![to_address.    to_string(), erc20_transfer_value.to_string()],
    )?;

    let erc20_transfer_tx = method_call.send().await?;

    //let erc20_transfer_tx = erc20_rw_contract.transfer(to_address, erc20_transfer_value);

    println!("ERC20 Transaction Receipt: {:?}", &erc20_transfer_tx);
    */

    let ierc20_rw_contract = IERC20::new(contract_address, signer.into());
    let ierc20_transfer_method = ierc20_rw_contract.transfer(to_address, erc20_transfer_value);
    let ierc20_transfer_tx = ierc20_transfer_method.send().await?;
    println!("IERC20 Transaction Receipt: {:?}", &ierc20_transfer_tx);

    /*
    '0xf3864301fd32e52e03904daceb80941daffffc7dce843f025e34705f4ef22a5c', // Smart contract sent from wallet
    '0xe7a4e28e0748f7484efe5cad6414ebcc11afc67b67922dc748958fcd12d9988f', // ETH sent from wallet
    '0xe7a4e28e0748f7484efe5cad6414ebcc11afc67b67922dc748958fcd12d9988c', // FALSE
    '0xedcc46917ab09b5d5b0f08f359061c5339d3d6eea0f1fee059ab842b41d08366', // RECV ETH
    '0xc3c166d417f22c21e28f8c8f3540aadabccafd64fd8d512ab91944a5b1276aa1', // Approve token spend limit
    '0xd042ed8f01b27ee31192cd7b46c2a3f5066bd31a4addf243cffcfeb795dedd4f', // SWAP Exact ETH for TOKENS
    '0x816df23c517b234d30e5504289ca40f66f9571b803b7a54e19bf4b724c67b299', // Approve WETH spend limit
    */

    let txnList = [
        "0x28eac16d0873e3cd24baa261e51f4a1ae4f92d96a94ed5fcc244346d85a6a91a", // ETH 0.2 recvd into wallet
        "0x1a99db4fd9783d9d2a8e7e359cd81745418b22210cab9abece78d4cd96f1f4dd", // NEXM 10,000,000 transfer() into wallet
        "0x279f831252e4c32022d82bd5afef1fdd51eb1c86d8417d52cb6251b87f110e21", // ETH 0.0001 sent from wallet
        "0x9579e0cb7a7fa16942868b9f731167463fdb9c848d83f6903bf143645b5143c5", // NEXM 1 transfer() from wallet
        "0xe7a4e28e0748f7484efe5cad6414ebcc11afc67b67922dc748958fcd12d9988c", // FALSE TXN
    ];

    verifyTransaction(
        &provider,
        "0x28eac16d0873e3cd24baa261e51f4a1ae4f92d96a94ed5fcc244346d85a6a91a",
    )
    .await;

    verifyTransaction(
        &provider,
        "0x1a99db4fd9783d9d2a8e7e359cd81745418b22210cab9abece78d4cd96f1f4dd",
    )
    .await;

    verifyTransaction(
        &provider,
        "0x9579e0cb7a7fa16942868b9f731167463fdb9c848d83f6903bf143645b5143c5",
    )
    .await;

    Ok(())
}

async fn verifyTransaction(
    provider: &Provider<Http>,
    //abi: &ethers::abi::Abi,
    txHash: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let hash: H256 = txHash.parse::<H256>()?;

    println!("running traces for txHash {}", txHash);
    let options: GethDebugTracingOptions = GethDebugTracingOptions::default();
    match provider.debug_trace_transaction(hash, options).await {
        Ok(trace) => {
            println!("trace ran");
            println!("trace {:?}", trace);
        }
        Err(x) => {
            println!("trace failedx {:?}", x);
        }
    };
    println!("");

    println!("fetching status for {:?}", hash);
    let txnStatus = provider.get_transaction(hash).await?.unwrap();
    println!("txStatus {:?}", txnStatus);

    println!("txStatus {:?}", txnStatus.input);
    if txnStatus.input.to_string() == "0x" {
        println!("Ethereum Transfer");
        println!("from {:?}", txnStatus.from);
        println!("to {:?}", txnStatus.to.unwrap());
        println!("value {:?}", ethers::utils::format_ether(txnStatus.value));
        println!("block number {}", txnStatus.block_number.unwrap());
        println!("block hash {}", txnStatus.block_hash.unwrap());
    } else {
        println!("Smartcontract interaction");
        //println!("input {:?}", txnStatus.input);
        println!("from {:?}", txnStatus.from);

        let decoded_data = TransferCall::decode(&txnStatus.input)?;
        println!(
            "input {:?}",
            decoded_data.recipient
        );
        println!(
            "value {:?}",
            ethers::utils::format_units(decoded_data.amount, 8)
        );
        println!("block number {}", txnStatus.block_number.unwrap());
        println!("block hash {}", txnStatus.block_hash.unwrap());
    }

    let block_hash: H256 = txnStatus.block_hash.unwrap();
    let block =  provider.get_block(block_hash).await;
    println!("block {:?}", block);

    println!("");
    println!("");

    Ok(())
}
