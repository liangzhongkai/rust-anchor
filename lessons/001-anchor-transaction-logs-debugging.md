# 001 - Anchor 测试中交易日志打印

> 基于对话总结的迭代文档，通过错误复盘来学习。

---

## 错误 1：无法在测试中查看交易执行日志

### 背景

在 Anchor 项目编写集成测试时，执行了 `program.methods.initialize().rpc()` 发起链上交易。需要查看该交易执行过程中的程序日志（如 `msg!` 输出、系统日志等），以便调试和验证程序行为。

### 错误

调用 `rpc()` 后，只能得到交易签名（transaction signature），无法直接获取或打印交易执行时产生的日志。测试输出中看不到程序内部的 `msg!` 或系统级日志信息。

### 错误原因

1. **API 设计**：Anchor 的 `rpc()` 方法返回的是 `Promise<string>`，即交易签名，不包含交易详情或日志。
2. **数据分离**：Solana 的交易日志存储在交易元数据（`meta.logMessages`）中，需要根据签名单独向 RPC 节点请求已确认的交易详情才能获取。

### 解决方案

在 `rpc()` 返回签名后，使用 `connection.getTransaction()` 拉取已确认的交易，再从 `meta.logMessages` 中读取并打印日志：

```typescript
const tx = await program.methods.initialize().rpc();
console.log("Your transaction signature", tx);

const connection = program.provider.connection;
const confirmedTx = await connection.getTransaction(tx, {
  commitment: "confirmed",
  maxSupportedTransactionVersion: 0,
});
if (confirmedTx?.meta?.logMessages) {
  console.log("Transaction logs:");
  confirmedTx.meta.logMessages.forEach((log) => console.log(log));
}
```

要点：
- `commitment: "confirmed"` 确保获取已确认的交易
- `maxSupportedTransactionVersion: 0` 兼容版本化交易（Versioned Transactions）

### 扩展

1. **封装工具函数**：将「获取并打印交易日志」抽成 `printTransactionLogs(txSignature, connection)`，在多个测试中复用。
2. **失败时自动打印**：在 `try/catch` 的 `catch` 中，若错误包含交易签名，可自动拉取并打印该交易的日志，便于排查失败原因。
3. **断言日志内容**：除打印外，可对 `logMessages` 做断言，验证关键日志是否存在，提升测试的确定性。
