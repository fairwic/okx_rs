use crate::api::api_trait::OkxApiTrait;
use crate::api::API_TRADE_PATH;
use crate::client::OkxClient;
use crate::dto::trade::trade_dto::{FeeRate, OrderPendingRespDto, OrderReqDto, OrderResDto};
use crate::dto::trade_dto::{CloseOrderReqDto, OrdListReqDto, OrderDetailRespDto};
use crate::error::Error;
use reqwest::Method;
use serde_json::json;

/// OKX交易API
/// 提供交易相关的API访问
#[derive(Debug)]
pub struct OkxTrade {
    /// API客户端
    client: OkxClient,
}

impl OkxApiTrait for OkxTrade {
    fn new(client: OkxClient) -> Self {
        OkxTrade { client }
    }
    fn client(&self) -> &OkxClient {
        &self.client
    }
}

impl OkxTrade {
    /// 下单
    pub async fn place_order(&self, order_params: OrderReqDto) -> Result<Vec<OrderResDto>, Error> {
        let path = format!("{}/order", API_TRADE_PATH);
        let body_str = serde_json::to_string(&order_params).map_err(Error::JsonError)?;
        self.client
            .send_request::<Vec<OrderResDto>>(Method::POST, &path, &body_str)
            .await
    }

    /// 批量下单
    pub async fn place_multiple_orders(
        &self,
        orders: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, Error> {
        let path = format!("{}/batch-orders", API_TRADE_PATH);
        let body_str = serde_json::to_string(&orders).map_err(Error::JsonError)?;
        self.client
            .send_request::<serde_json::Value>(Method::POST, &path, &body_str)
            .await
    }

    /// 撤单
    pub async fn cancel_order(
        &self,
        inst_id: &str,
        ord_id: Option<&str>,
        cl_ord_id: Option<&str>,
    ) -> Result<serde_json::Value, Error> {
        let path = format!("{}/cancel-order", API_TRADE_PATH);

        let mut body = json!({
            "instId": inst_id,
        });

        if let Some(order_id) = ord_id {
            body["ordId"] = json!(order_id);
        }

        if let Some(client_order_id) = cl_ord_id {
            body["clOrdId"] = json!(client_order_id);
        }

        let body_str = serde_json::to_string(&body).map_err(Error::JsonError)?;
        self.client
            .send_request::<serde_json::Value>(Method::POST, &path, &body_str)
            .await
    }

    /// 批量撤单
    pub async fn cancel_multiple_orders(
        &self,
        orders: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, Error> {
        let path = format!("{}/cancel-batch-orders", API_TRADE_PATH);
        let body_str = serde_json::to_string(&orders).map_err(Error::JsonError)?;
        self.client
            .send_request::<serde_json::Value>(Method::POST, &path, &body_str)
            .await
    }

    /// 修改订单
    pub async fn amend_order(
        &self,
        inst_id: &str,
        ord_id: Option<&str>,
        cl_ord_id: Option<&str>,
        req_id: Option<&str>,
        new_sz: Option<&str>,
        new_px: Option<&str>,
    ) -> Result<serde_json::Value, Error> {
        let path = format!("{}/amend-order", API_TRADE_PATH);

        let mut body = json!({
            "instId": inst_id,
        });

        if let Some(order_id) = ord_id {
            body["ordId"] = json!(order_id);
        }

        if let Some(client_order_id) = cl_ord_id {
            body["clOrdId"] = json!(client_order_id);
        }

        if let Some(request_id) = req_id {
            body["reqId"] = json!(request_id);
        }

        if let Some(new_size) = new_sz {
            body["newSz"] = json!(new_size);
        }

        if let Some(new_price) = new_px {
            body["newPx"] = json!(new_price);
        }

        let body_str = serde_json::to_string(&body).map_err(Error::JsonError)?;
        self.client
            .send_request::<serde_json::Value>(Method::POST, &path, &body_str)
            .await
    }

    /// 获取订单信息
    pub async fn get_order_details(
        &self,
        inst_id: &str,
        ord_id: Option<&str>,
        cl_ord_id: Option<&str>,
    ) -> Result<Vec<OrderDetailRespDto>, Error> {
        let mut path = format!("{}/order?instId={}", API_TRADE_PATH, inst_id);

        if let Some(order_id) = ord_id {
            path.push_str(&format!("&ordId={}", order_id));
        }

        if let Some(client_order_id) = cl_ord_id {
            path.push_str(&format!("&clOrdId={}", client_order_id));
        }

        self.client
            .send_request::<Vec<OrderDetailRespDto>>(Method::GET, &path, "")
            .await
    }

    /// 获取未成交订单列表
    pub async fn get_pending_orders(
        &self,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
        ord_type: Option<&str>,
        state: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<OrderPendingRespDto>, Error> {
        let mut path = format!("{}/orders-pending", API_TRADE_PATH);
        let mut query_params = vec![];

        if let Some(it) = inst_type {
            query_params.push(format!("instType={}", it));
        }

        if let Some(id) = inst_id {
            query_params.push(format!("instId={}", id));
        }

        if let Some(ot) = ord_type {
            query_params.push(format!("ordType={}", ot));
        }

        if let Some(s) = state {
            query_params.push(format!("state={}", s));
        }

        if let Some(a) = after {
            query_params.push(format!("after={}", a));
        }

        if let Some(b) = before {
            query_params.push(format!("before={}", b));
        }

        if let Some(l) = limit {
            query_params.push(format!("limit={}", l));
        }

        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }

        self.client
            .send_request::<Vec<OrderPendingRespDto>>(Method::GET, &path, "")
            .await
    }

    // 获取最近7天挂单，且完成的订单数据，包括7天以前挂单，但近7天才成交的订单数据。按照订单创建时间倒序排序。
    // 已经撤销的未成交单 只保留2小时
    // 限速：40次/2s
    // 限速规则：User ID
    pub async fn get_order_history(
        &self,
        params: OrdListReqDto,
    ) -> Result<Vec<OrderDetailRespDto>, Error> {
        let mut path = format!("{}/orders-history?instType={}", API_TRADE_PATH, params.inst_type);

        if let Some(id) = params.inst_id {
            path.push_str(&format!("&instId={}", id));
        }

        if let Some(ot) = params.ord_type {
            path.push_str(&format!("&ordType={}", ot));
        }

        if let Some(s) = params.state {
            path.push_str(&format!("&state={}", s));
        }

        if let Some(a) = params.after {
            path.push_str(&format!("&after={}", a));
        }

        if let Some(b) = params.before {
            path.push_str(&format!("&before={}", b));
        }

        if let Some(l) = params.limit {
            path.push_str(&format!("&limit={}", l));
        }

        self.client
            .send_request::<Vec<OrderDetailRespDto>>(Method::GET, &path, "")
            .await
    }

    ///GET / 获取历史订单记录（近三个月）
    /// 获取最近3个月挂单，且完成的订单数据，包括3个月以前挂单，但近3个月才成交的订单数据。按照订单创建时间倒序排序。
    /// 限速：20次/2s
    /// 限速规则：User ID
    pub async fn get_order_history_archive(
        &self,
        params: OrdListReqDto,
    ) -> Result<Vec<OrderDetailRespDto>, Error> {
        let mut path = format!(
            "{}/orders-history-archive?instType={}",
            API_TRADE_PATH, params.inst_type
        );

        if let Some(id) = params.inst_id {
            path.push_str(&format!("&instId={}", id));
        }

        if let Some(ot) = params.ord_type {
            path.push_str(&format!("&ordType={}", ot));
        }

        if let Some(s) = params.state {
            path.push_str(&format!("&state={}", s));
        }

        if let Some(a) = params.after {
            path.push_str(&format!("&after={}", a));
        }

        if let Some(b) = params.before {
            path.push_str(&format!("&before={}", b));
        }

        if let Some(l) = params.limit {
            path.push_str(&format!("&limit={}", l));
        }

        self.client
            .send_request::<Vec<OrderDetailRespDto>>(Method::GET, &path, "")
            .await
    }

    /// 获取成交明细
    pub async fn get_fills(
        &self,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
        ord_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> Result<serde_json::Value, Error> {
        let mut path = format!("{}/fills", API_TRADE_PATH);
        let mut query_params = vec![];

        if let Some(it) = inst_type {
            query_params.push(format!("instType={}", it));
        }

        if let Some(id) = inst_id {
            query_params.push(format!("instId={}", id));
        }

        if let Some(oid) = ord_id {
            query_params.push(format!("ordId={}", oid));
        }

        if let Some(a) = after {
            query_params.push(format!("after={}", a));
        }

        if let Some(b) = before {
            query_params.push(format!("before={}", b));
        }

        if let Some(l) = limit {
            query_params.push(format!("limit={}", l));
        }

        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }

        self.client
            .send_request::<serde_json::Value>(Method::GET, &path, "")
            .await
    }

    /// 获取交易产品费率
    pub async fn get_fee_rates(
        &self,
        inst_type: &str,
        inst_id: Option<&str>,
        uly: Option<&str>,
    ) -> Result<Vec<FeeRate>, Error> {
        let mut path = format!("{}/trade-fee?instType={}", API_TRADE_PATH, inst_type);

        if let Some(id) = inst_id {
            path.push_str(&format!("&instId={}", id));
        }

        if let Some(underlying) = uly {
            path.push_str(&format!("&uly={}", underlying));
        }

        self.client
            .send_request::<Vec<FeeRate>>(Method::GET, &path, "")
            .await
    }

    /// 平仓 (从顶层trade模块合并)
    pub async fn close_position(
        &self,
        params: &CloseOrderReqDto,
    ) -> Result<serde_json::Value, Error> {
        let path = format!("{}/close-position", API_TRADE_PATH);

        let body_str = serde_json::to_string(params).map_err(Error::JsonError)?;
        self.client
            .send_request::<serde_json::Value>(Method::POST, &path, &body_str)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_place_order() {
        // 仅作为示例，实际测试需要提供有效的值
        let trade = OkxTrade::from_env().expect("无法从环境变量创建交易API");
        let result = trade
            .place_order(OrderReqDto {
                inst_id: "BTC-USDT".to_string(),
                side: "buy".to_string(),
                ord_type: "limit".to_string(),
                sz: "0.001".to_string(),
                td_mode: "cash".to_string(),
                px: Some("20000".to_string()),
                pos_side: None,
                cl_ord_id: None,
                tag: None,
                reduce_only: None,
                ccy: None,
                px_usd: None,
                px_vol: None,
                tgt_ccy: None,
                ban_amend: None,
                quick_mgn_type: None,
                stp_id: None,
                stp_mode: None,
                attach_algo_ords: None,
            })
            .await;

        println!("Place order result: {:?}", result);
    }
}
