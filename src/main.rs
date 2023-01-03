use ethers::prelude::{abigen, Abigen};
use ethers::providers::{Http, Provider};
use ethers::{prelude::*, types::U256, utils};
use std::convert::TryFrom;

//type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

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
    let erc20_contract =
        ethers::contract::Contract::new(contract_address, erc20_contract_abi.clone(), provider);

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

    let ierc20_rw_contract = IERC20::new(contract_address, signer.into());
    let ierc20_transfer_method = ierc20_rw_contract.transfer(to_address, erc20_transfer_value);
    let ierc20_transfer_tx = ierc20_transfer_method.send().await?;
    println!("IERC20 Transaction Receipt: {:?}", &ierc20_transfer_tx);

    Ok(())
}
