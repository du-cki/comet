import { sleep } from '@/utils'

import type {
    FileT,
    RequestFileResponse,
} from '@/lib/comet/types'


class Client {
    conn: WebSocket
    requests: { [key: string]: (data: any) => void } = {}
    onmessage?: (data: FileT) => any

    constructor(baseUrl: string) {
        this.conn = new WebSocket(baseUrl)

        this.conn.onmessage = (ev) => {
            const data = JSON.parse(ev.data)

            const request_id = data.request_id
            if (request_id) {
                this.requests[request_id](data)
                delete this.requests[request_id]
            } else if (this.onmessage) {
                this.onmessage(data)
            }
        }
    }

    private generateRequestId() {
        return Math.random().toString(36).substring(2)
    }

    async requestFiles(directory = '/'): Promise<RequestFileResponse> {
        const requestId = this.generateRequestId()

        if (this.conn.readyState !== WebSocket.OPEN) {
            await sleep(.1);
            return this.requestFiles(directory)
        }

        this.conn.send(JSON.stringify({
            event: 'query_folder',
            data: directory,
            request_id: requestId,
        }))

        return new Promise((resolve, _) => {
            this.requests[requestId] = resolve
        })
    }
}

export default Client;