Definition = object passes a request along a chain of potential handlers until one of them handles the request.

Interface = Each handler handles the request and optionally implements a successor link.

Advantages 
* Decoupling = of sender and receivers
* Open/Closed Principle = new handlers can be added/removed at runtime

Disadvantages
* Request may not be handled = the request may end up being unhandled.

Alternatives
* Could be done concurrently 

```
type Handler interface {
    SetNext(handler Handler) Handler
    HandleRequest(message string) string
}

type BaseHandler struct {
    next Handler
}
func (h *BaseHandler) SetNext(handler Handler) Handler {
    h.next = handler
    return handler
}
func (h *BaseHandler) HandleRequest(message string) string{
    if h.next != nil {
        return h.next.HandleRequest(message)
    }
    return message
}

type FilterHandler struct {
    BaseHandler
}
func (h *InfoHandler) HandleRequest(message string) string {
    processed := strings.Map(func(r rune) rune { // mapping function
        if strings.ContainsRune("0123456789.,!?;", r) {
            return -1
        }
        return r
    }, message)
    return h.BaseHandler.HandleRequest(processed)
}
// ...
func main(){
    filter := &FilterHandler{}
    format := &FormatHandler{}
    logger := &LogHandler{}

    filter.SetNext(format).SetNext(logger)
}
```