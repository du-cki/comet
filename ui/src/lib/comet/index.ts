import { sleep } from '@/utils'

import type { RequestFileResponse } from '@/lib/comet/types'

class Client {
  conn?: WebSocket
  baseUrl: string
  requests: { [key: string]: (data: any) => void } = {}
  onmessage: (data: object) => any
  onopen: () => any
  onclose: () => any

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl
    this.connect()
      .then((socket) => {
        this.conn = socket
      })
      .catch((err) => {
        // won't ever actually happen in PROD since the socket would be served
        // by the same app the site is running on.
        throw err
      })

    this.onmessage = () => {}
    this.onopen = () => {}
    this.onclose = () => {}
  }

  private generateRequestId() {
    return Math.random().toString(36).substring(2)
  }

  connect(): Promise<WebSocket> {
    const socket = new WebSocket(this.baseUrl)

    // if socket is opened only, add the event listeners.
    // this is kinda jank, but the default `WebSocket` constructor
    // is a bit weird, but I don't care enough to install another
    // library for it.
    return new Promise((resolve, reject) => {
      socket.addEventListener('open', () => {
        // since it is already opened, we can't use `onopen`. So
        // we call it manually.
        this.onopen()

        // we only need an `onclose` only if the socket actually closes
        // after connecting.
        socket.onclose = () => this.onclose()

        socket.onmessage = (ev) => {
          const data = JSON.parse(ev.data)

          const request_id = data.request_id
          if (request_id) {
            this.requests[request_id](data)
            delete this.requests[request_id]
          } else if (this.onmessage) {
            this.onmessage(data)
          }
        }

        resolve(socket)
      })

      socket.addEventListener('close', () => {
        socket.close()
        reject(socket)
      })
    })
  }

  reconnect() {
    const IntId = setInterval(() => {
      this.connect().then((socket) => {
        this.conn = socket
        clearInterval(IntId)
      })
    }, 1500)
  }

  async requestFiles(directory = '/'): Promise<RequestFileResponse> {
    if (this.conn?.readyState !== WebSocket.OPEN) {
      await sleep(0.1)

      return this.requestFiles(directory)
    }

    const requestId = this.generateRequestId()

    this.conn!.send(
      JSON.stringify({
        event: 'query_folder',
        data: directory,
        request_id: requestId
      })
    )

    return new Promise((resolve, _) => {
      this.requests[requestId] = resolve
    })
  }
}

export default Client
