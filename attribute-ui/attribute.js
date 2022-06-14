document.addEventListener("DOMContentLoaded", function() {
  const hostToken = window.location.pathname.split('/').pop();
  const rootElement = document.getElementById('root');

  function fetchAndRender() {
    return fetch(`session_info/${hostToken}`, {
        credentials: 'include',
      })
      .then((r) => {
        if (r.status === 200 || r.status === 401 || r.status === 404) {
          r.text().then((html) => {
            rootElement.innerHTML = html;
          });
        }

        return r.status;
      });
  }

  function listenForEvents() {
    const source = new EventSource(`live/session_info/${hostToken}`, { withCredentials: true });

    source.onmessage = (event) => {
      if (event.data) {
        fetchAndRender();
      }
    };

    source.onerror = () => {
      source.close();
      setTimeout(listenForEvents, 5 * 1000);
    };
  }

  function poll() {
    if (!document.hidden) {
      fetchAndRender().then((status) => {
        if (status === 401) {
          // keep polling until user is logged in
          setTimeout(poll, 5 * 1000);
        } else {
          // start listinging for server updates
          listenForEvents();
        }
      });
    } else {
      setTimeout(poll, 5 * 1000);
    }
  }

  // poll until user is logged in
  poll();
});
