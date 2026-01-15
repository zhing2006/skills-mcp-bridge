use rmcp::{
    handler::client::ClientHandler,
    model::{ClientInfo, LoggingMessageNotificationParam, ProgressNotificationParam},
    service::{NotificationContext, RoleClient},
};

#[derive(Clone)]
pub(crate) struct ClientHandlerImpl {
    info: ClientInfo,
    emit_notifications: bool,
}

impl ClientHandlerImpl {
    pub(crate) fn new(info: ClientInfo, emit_notifications: bool) -> Self {
        Self {
            info,
            emit_notifications,
        }
    }
}

impl ClientHandler for ClientHandlerImpl {
    fn on_progress(
        &self,
        params: ProgressNotificationParam,
        _context: NotificationContext<RoleClient>,
    ) -> impl std::future::Future<Output = ()> + Send + '_ {
        let emit = self.emit_notifications;
        async move {
            if !emit {
                return;
            }

            let mut line = format!("[progress] {}", params.progress);
            if let Some(total) = params.total {
                line.push_str(&format!("/{}", total));
            }
            if let Some(message) = params.message
                && !message.is_empty()
            {
                line.push(' ');
                line.push_str(&message);
            }
            println!("{line}");
        }
    }

    fn on_logging_message(
        &self,
        params: LoggingMessageNotificationParam,
        _context: NotificationContext<RoleClient>,
    ) -> impl std::future::Future<Output = ()> + Send + '_ {
        let emit = self.emit_notifications;
        async move {
            if !emit {
                return;
            }

            let level = format!("{:?}", params.level).to_ascii_lowercase();
            let mut line = format!("[log:{level}]");
            if let Some(logger) = params.logger
                && !logger.is_empty()
            {
                line.push(' ');
                line.push_str(&logger);
            }
            let data = params.data.to_string();
            if !data.is_empty() {
                line.push(' ');
                line.push_str(&data);
            }
            println!("{line}");
        }
    }

    fn get_info(&self) -> ClientInfo {
        self.info.clone()
    }
}
