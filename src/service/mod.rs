mod user;
mod friendships;

use once_cell::sync::Lazy;
use rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;
use futures_util::TryFutureExt;
pub use user::*;
pub use friendships::*;

pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::service::CONTEXT.rb.clone()
    };
}

pub fn init_rbatis() -> RBatis {
    let rbatis = RBatis::new();
    return rbatis;
}

pub struct ServiceContext {
    pub rb: RBatis,
}

impl Default for ServiceContext {
    fn default() -> Self {
        ServiceContext {
            rb: init_rbatis(),
        }
    }
}
impl ServiceContext {
    /// init database pool
    pub async fn init_pool(&self) {
        log::info!(
            "[abs_admin] rbatis pool init ()...",
        );
        self.rb
            .init(MysqlDriver{}, "mysql://kinwe:hx19870527@127.0.0.1:3306/test")
            .expect("[abs_admin] rbatis pool init fail!");
    }
}
