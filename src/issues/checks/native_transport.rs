use crate::issues::issue::Issue;

fn pre_1_12_native_transport_java_9(entry_text: &str) -> Option<Issue> {
    // This could be improved to reduce false positives with the stacktrace parser when more data is available
    entry_text.contains("at io.netty.channel.epoll").then_some(Issue::NettyJavaAbove8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_pre_1_12_native_transport_java_9() {
        // Text starts after header
        let text = "[00:15:19] [spark-async-sampler-worker-thread/WARN]: Timed out waiting for world statistics
[00:15:55] [Netty Epoll Server IO #1/WARN]: Decoding error for packet:class net.minecraft.class_2889
java.lang.IndexOutOfBoundsException: readerIndex(1) + length(1) exceeds writerIndex(1): PooledUnsafeDirectByteBuf(ridx: 1, widx: 1, cap: 1)
	at io.netty.buffer.AbstractByteBuf.checkReadableBytes0(AbstractByteBuf.java:1442) ~[netty-buffer-4.1.82.Final.jar:?]
	at io.netty.buffer.AbstractByteBuf.readByte(AbstractByteBuf.java:730) ~[netty-buffer-4.1.82.Final.jar:?]
	at net.minecraft.network.PacketByteBuf.readByte(PacketByteBuf.java:1303) ~[server-intermediary.jar:?]
	at net.minecraft.network.PacketByteBuf.readVarInt(PacketByteBuf.java:530) ~[server-intermediary.jar:?]
	at net.minecraft.network.packet.c2s.handshake.HandshakeC2SPacket.<init>(HandshakeC2SPacket.java:27) ~[server-intermediary.jar:?]
	at net.minecraft.network.NetworkState$PacketHandler.createPacket(class_2539.java:459) ~[server-intermediary.jar:?]
	at net.minecraft.network.NetworkState.getPacketHandler(NetworkState.java:522) ~[server-intermediary.jar:?]
	at net.minecraft.network.DecoderHandler.redirect$ccd000$connectivity$checkDecodingError(DecoderHandler.java:563) ~[server-intermediary.jar:?]
	at net.minecraft.network.DecoderHandler.decode(DecoderHandler.java:32) ~[server-intermediary.jar:?]
	at io.netty.handler.codec.ByteToMessageDecoder.decodeRemovalReentryProtection(ByteToMessageDecoder.java:519) ~[netty-codec-4.1.82.Final.jar:?]
	at io.netty.handler.codec.ByteToMessageDecoder.callDecode(ByteToMessageDecoder.java:458) ~[netty-codec-4.1.82.Final.jar:?]
	at io.netty.handler.codec.ByteToMessageDecoder.channelRead(ByteToMessageDecoder.java:280) ~[netty-codec-4.1.82.Final.jar:?]
	at io.netty.channel.AbstractChannelHandlerContext.invokeChannelRead(AbstractChannelHandlerContext.java:379) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.channel.AbstractChannelHandlerContext.invokeChannelRead(AbstractChannelHandlerContext.java:365) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.channel.AbstractChannelHandlerContext.fireChannelRead(AbstractChannelHandlerContext.java:357) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.handler.codec.ByteToMessageDecoder.fireChannelRead(ByteToMessageDecoder.java:336) ~[netty-codec-4.1.82.Final.jar:?]
	at io.netty.handler.codec.ByteToMessageDecoder.channelRead(ByteToMessageDecoder.java:308) ~[netty-codec-4.1.82.Final.jar:?]
	at io.netty.channel.AbstractChannelHandlerContext.invokeChannelRead(AbstractChannelHandlerContext.java:379) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.channel.AbstractChannelHandlerContext.invokeChannelRead(AbstractChannelHandlerContext.java:365) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.channel.AbstractChannelHandlerContext.fireChannelRead(AbstractChannelHandlerContext.java:357) ~[netty-transport-4.1.82.Final.jar:?]
	at net.minecraft.network.LegacyQueryHandler.channelRead(LegacyQueryHandler.java:95) ~[server-intermediary.jar:?]
	at io.netty.channel.AbstractChannelHandlerContext.invokeChannelRead(AbstractChannelHandlerContext.java:379) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.channel.AbstractChannelHandlerContext.invokeChannelRead(AbstractChannelHandlerContext.java:365) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.channel.AbstractChannelHandlerContext.fireChannelRead(AbstractChannelHandlerContext.java:357) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.handler.timeout.IdleStateHandler.channelRead(IdleStateHandler.java:286) ~[netty-handler-4.1.82.Final.jar:?]
	at io.netty.channel.AbstractChannelHandlerContext.invokeChannelRead(AbstractChannelHandlerContext.java:379) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.channel.AbstractChannelHandlerContext.invokeChannelRead(AbstractChannelHandlerContext.java:365) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.channel.AbstractChannelHandlerContext.fireChannelRead(AbstractChannelHandlerContext.java:357) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.channel.DefaultChannelPipeline$HeadContext.channelRead(DefaultChannelPipeline.java:1410) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.channel.AbstractChannelHandlerContext.invokeChannelRead(AbstractChannelHandlerContext.java:379) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.channel.AbstractChannelHandlerContext.invokeChannelRead(AbstractChannelHandlerContext.java:365) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.channel.DefaultChannelPipeline.fireChannelRead(DefaultChannelPipeline.java:919) ~[netty-transport-4.1.82.Final.jar:?]
	at io.netty.channel.epoll.AbstractEpollStreamChannel$EpollStreamUnsafe.epollInReady(AbstractEpollStreamChannel.java:800) ~[netty-transport-classes-epoll-4.1.82.Final.jar:?]
	at io.netty.channel.epoll.EpollEventLoop.processReady(EpollEventLoop.java:499) ~[netty-transport-classes-epoll-4.1.82.Final.jar:?]
	at io.netty.channel.epoll.EpollEventLoop.run(EpollEventLoop.java:397) ~[netty-transport-classes-epoll-4.1.82.Final.jar:?]
	at io.netty.util.concurrent.SingleThreadEventExecutor$4.run(SingleThreadEventExecutor.java:997) ~[netty-common-4.1.82.Final.jar:?]
	at io.netty.util.internal.ThreadExecutorMap$2.run(ThreadExecutorMap.java:74) ~[netty-common-4.1.82.Final.jar:?]
	at java.lang.Thread.run(Thread.java:840) ~[?:?]
[00:15:55] [Netty Epoll Server IO #1/WARN]: <------ Packet Data Export: ------>
[00:15:55] [Netty Epoll Server IO #1/WARN]: Packet:class_2540 
";
        let issue = pre_1_12_native_transport_java_9(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::NettyJavaAbove8);
    }
}