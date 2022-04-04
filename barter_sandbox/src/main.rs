use parking_lot::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::mpsc;
use uuid::Uuid;
use barter::execution::Fees;
use barter::engine::Engine;
use barter::statistic::summary::Initialiser;
use barter::execution::simulated::SimulatedExecution;
use barter::strategy::strategy::RSIStrategy;
use barter::Market;
use barter::event::EventTx;
use barter::engine::trader::Trader;
use barter::portfolio::risk::DefaultRisk;
use barter::portfolio::allocator::DefaultAllocator;
use barter::portfolio::repository::in_memory::InMemoryRepository;
use barter::portfolio::portfolio::MetaPortfolio;
use barter::data::handler::historical::{HistoricalCandleHandler, HistoricalDataLego};
use barter::execution::simulated::Config as ExecutionConfig;
use barter::strategy::strategy::Config as StrategyConfig;
use barter::statistic::summary::trading::{Config as StatisticConfig, TradingSummary};
    use barter_data::test_util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup Logger & Load Config For Engine & Trader Instances Here

    // Create channel to distribute Commands to the Engine & it's Traders (eg/ Command::Terminate)
    let (command_tx, command_rx) = mpsc::channel(20);
    
    // Create Event channel to listen to all Engine Events in real-time
    let (event_tx, event_rx) = mpsc::unbounded_channel();
    let event_tx = EventTx::new(event_tx);
    
    // Generate unique identifier to associate an Engine's components
    let engine_id = Uuid::new_v4();
    
    // Create the Market(s) to be traded on (1-to-1 relationship with a Trader) 
    let market = Market::new("binance", "btc_usdt".to_owned());
    
    // Build global shared-state MetaPortfolio (1-to-1 relationship with an Engine)
    let portfolio = Arc::new(Mutex::new(
        MetaPortfolio::builder()
            .engine_id(engine_id)
            .markets(vec![market.clone()])
            .starting_cash(10_000.0)
            .repository(InMemoryRepository::new())
            .allocation_manager(DefaultAllocator { default_order_value: 100.0 })
            .risk_manager(DefaultRisk {})
            .statistic_config(StatisticConfig { 
                starting_equity: 10_000.0, 
                trading_days_per_year: 365, 
                risk_free_return: 0.0 
            })
            .build_and_init()
            .expect("failed to build & initialise MetaPortfolio"),
    ));
    
    // Build Trader(s)
    let mut traders = Vec::new();
    
    // Create channel for each Trader so the Engine can distribute Commands to it
    let (trader_command_tx, trader_command_rx) = mpsc::channel(10);

    traders.push(
        Trader::builder()
            .engine_id(engine_id)
            .market(market.clone())
            .command_rx(trader_command_rx)
            .event_tx(event_tx.clone())
            .portfolio(Arc::clone(&portfolio))
            .data(HistoricalCandleHandler::new(HistoricalDataLego {
                exchange: "binance",
                symbol: "btcusdt".to_string(),
                candles: vec![test_util::candle()].into_iter()
            }))
            .strategy(RSIStrategy::new(StrategyConfig { rsi_period: 14 }))
            .execution(SimulatedExecution::new(ExecutionConfig {
                simulated_fees_pct: Fees {
                        exchange: 0.1,
                        slippage: 0.05,
                        network: 0.0,}
                }))
            .build()
            .expect("failed to build trader")
    );
    
    // Build Engine (1-to-many relationship with Traders)
    
    // Create HashMap<Market, trader_command_tx> so Engine can route Commands to Traders 
    let trader_command_txs = HashMap::from_iter([(market, trader_command_tx)]);
    
    let engine = Engine::builder()
        .engine_id(engine_id)
        .command_rx(command_rx)
        .portfolio(portfolio)
        .traders(traders)
        .trader_command_txs(trader_command_txs)
        .statistics_summary(TradingSummary::init(StatisticConfig {
            starting_equity: 1000.0,
            trading_days_per_year: 365,
            risk_free_return: 0.0
        }))
        .build()
        .expect("failed to build engine");
        
    // Listen to Engine Events & do something with them
    // tokio::spawn(listen_to_events(event_rx)); 
        
    // --- Run Trading Session Until Remote Shutdown OR Data Feed ends naturally (ie/ backtest) ---
    engine.run().await;
    Err("Error...".into())
}
