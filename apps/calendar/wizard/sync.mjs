/** Transport between a session and the calendar sync server (server/calendar_server.py).
 * Plain fetch + long-polling, so it runs unchanged in the browser wizard and in
 * Node tests. The session stays pure: this module only submits
 * `session.pending` and feeds server records to `applyRecords`. */
import {adoptSnapshot, applyRecords} from './service.mjs';

export class SyncError extends Error { constructor(message, status, body) { super(message); this.status = status; this.body = body; } }

export function createSyncClient({baseUrl, token, device, fetchImpl = globalThis.fetch}) {
  if (!baseUrl || !token || !device) throw new TypeError('baseUrl, token and device are required');
  const base = baseUrl.replace(/\/$/, '');
  async function request(method, path, body, signal) {
    let response;
    try {
      response = await fetchImpl(base + path, {method, signal, headers: {authorization: 'Bearer ' + token, ...(body ? {'content-type': 'application/json'} : {})}, body: body ? JSON.stringify(body) : undefined});
    } catch (error) { throw new SyncError('network: ' + error.message, 0, null); }
    const text = await response.text();
    let parsed = null; try { parsed = text ? JSON.parse(text) : null; } catch { parsed = null; }
    if (!response.ok) throw new SyncError(`${method} ${path}: ${response.status}`, response.status, parsed);
    return parsed;
  }
  return {
    device,
    bootstrap: signal => request('GET', '/v1/state', null, signal),
    submit: (op, signal) => request('POST', '/v1/ops', op, signal),
    pull: (since, wait = 0, signal) => request('GET', `/v1/ops?since=${since}&wait=${wait}`, null, signal),
  };
}

/** Keeps one session in step with the server. `current` is always the latest session;
 * `sync()` pushes what is pending and pulls what is new; `follow()` long-polls until aborted. */
export class SessionSync {
  constructor(session, client, {onChange = () => {}} = {}) { this.current = session; this.client = client; this.onChange = onChange; this.sent = new Set(); }
  update(session) { if (session !== this.current) { this.current = session; this.onChange(session); } return session; }
  async connect(signal) { return this.update(adoptSnapshot(this.current, await this.client.bootstrap(signal))); }
  async push(signal) {
    const records = [];
    for (const op of this.current.pending) {
      if (this.sent.has(op.id)) continue;
      records.push(await this.client.submit(op, signal));
      this.sent.add(op.id);
    }
    return records;
  }
  async pull(wait = 0, signal) {
    const {records} = await this.client.pull(this.current.remote.seq, wait, signal);
    return this.update(applyRecords(this.current, records));
  }
  async sync(signal) { await this.push(signal); return this.pull(0, signal); }
  async follow({signal, wait = 25} = {}) {
    while (!signal?.aborted) {
      try { await this.sync(signal); await this.pull(wait, signal); }
      catch (error) { if (signal?.aborted) return; if (!(error instanceof SyncError)) throw error; await new Promise(resolve => setTimeout(resolve, 1000)); }
    }
  }
}
