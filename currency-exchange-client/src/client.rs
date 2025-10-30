use std::panic::AssertUnwindSafe;
use clap::{Parser, Subcommand};
use serde::Serialize;

#[derive(Parser, Serialize, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct CreateUserArgs {
    #[arg(long)]
    pub username: String,
    #[arg(long)]
    pub email: String,
    #[arg(long)]
    pub password: String,
    #[arg(long)]
    pub firstname: String,
    #[arg(long)]
    pub middlename: String,
    #[arg(long)]
    pub lastname: String,
}

#[derive(Parser, Serialize, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct LoginUserArgs {
    #[arg(long)]
    pub username: String,
    #[arg(long)]
    pub password: String,
}

#[derive(Parser, Serialize, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct CreateCurrencyArgs {
    #[arg(long)]
    pub currency_code: String,
    #[arg(long)]
    pub auth_token: String,
}

#[derive(Parser, Serialize, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct CreateWalletArgs {
    #[arg(long)]
    pub user_id: i32,
    #[arg(long)]
    pub currency_id: i32,
    #[arg(long)]
    pub auth_token: String,
}

#[derive(Parser, Serialize, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct AddCurrencyArgs {
    #[arg(long)]
    pub user_id: i32,
    #[arg(long)]
    pub currency_id: i32,
    #[arg(long)]
    pub auth_token: String,
}

#[derive(Parser, Serialize, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct ShowBuyOrdersArgs {
    #[arg(long)]
    pub orders: i64,
    #[arg(long)]
    auth_token: String,
}

#[derive(Parser, Serialize, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct ShowSellOrdersArgs {
    #[arg(long)]
    pub orders: i64,
    #[arg(long)]
    auth_token: String,
}

#[derive(Parser, Serialize, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct CreateBuyOrderArgs {
    #[arg(long)]
    pub issuer_id: i32,
    #[arg(long)]
    pub buy_amount: i32,
    #[arg(long)]
    pub buy_currency_id: i32,
    #[arg(long)]
    pub sell_currency_id: i32,
    #[arg(long)]
    pub expiry_days: i32,
    #[arg(long)]
    auth_token: String,
}

#[derive(Parser, Serialize, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct CreateSellOrderArgs {
    #[arg(long)]
    pub issuer_id: i32,
    #[arg(long)]
    pub sell_amount: i32,
    #[arg(long)]
    pub sell_currency_id: i32,
    #[arg(long)]
    pub buy_currency_id: i32,
    #[arg(long)]
    pub expiry_days: i32,
    #[arg(long)]
    auth_token: String,
}

#[derive(Parser, Serialize, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct BuyCurrencyArgs {
    #[arg(long)]
    pub sum: i32,
    #[arg(long)]
    pub rate: f32,
    #[arg(long)]
    pub order_issuer_id: i32,
    #[arg(long)]
    pub incoming_currency_id: i32,
    #[arg(long)]
    pub outgoing_currency_id: i32,
    #[arg(long)]
    pub auth_token: String,
}

#[derive(Parser, Serialize, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct SellCurrencyArgs {
    #[arg(long)]
    pub sum: i32,
    #[arg(long)]
    pub rate: f32,
    #[arg(long)]
    pub order_issuer_id: i32,
    #[arg(long)]
    pub incoming_currency_id: i32,
    #[arg(long)]
    pub outgoing_currency_id: i32,
    #[arg(long)]
    pub auth_token: String,
}

#[derive(Parser, Serialize, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct ListCurrenciesArgs {
    #[arg(long)]
    pub auth_token: String
}

#[derive(Subcommand)]
pub enum UserCommands {
    Create {
        #[command(flatten)]
        args: CreateUserArgs
    },
    Login {
        #[command(flatten)]
        args: LoginUserArgs
    }
}

#[derive(Subcommand)]
pub enum ApiCommands {
    CreateCurrency {
        #[command(flatten)]
        args: CreateCurrencyArgs
    },
    CreateWallet {
        #[command(flatten)]
        args: CreateWalletArgs
    },
    AddCurrencyForWallet {
        #[command(flatten)]
        args: AddCurrencyArgs
    },
    BuyOrders {
        #[command(flatten)]
        args: ShowBuyOrdersArgs
    },
    SellOrders {
        #[command(flatten)]
        args: ShowSellOrdersArgs
    },
    CreateBuyOrder {
        #[command(flatten)]
        args: CreateBuyOrderArgs
    },
    CreateSellOrder {
        #[command(flatten)]
        args: CreateSellOrderArgs
    },
    BuyCurrency {
        #[command(flatten)]
        args: BuyCurrencyArgs
    },
    SellCurrency {
        #[command(flatten)]
        args: SellCurrencyArgs
    },
    ListCurrencies {
        #[command(flatten)]
        args: ListCurrenciesArgs
    }
}

#[derive(Parser)]
#[command(name = "cli")]
pub enum CliCommands {
    Users {
        #[command(subcommand)]
        command: UserCommands,
    },
    Api {
        #[command(subcommand)]
        command: ApiCommands
    }
}