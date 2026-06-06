type SplitPaneOptions = {
  initialLeftPaneWidth?: number;
  minLeftPaneWidth: number;
  minRightPaneWidth: number;
  dividerWidth?: number;
};

export function createSplitPane({
  initialLeftPaneWidth = 440,
  minLeftPaneWidth,
  minRightPaneWidth,
  dividerWidth = 12
}: SplitPaneOptions) {
  let frameElement: HTMLElement | undefined = $state();
  let leftPaneWidth = $state(initialLeftPaneWidth);

  const maxLeftPaneWidth = $derived.by(() => {
    if (!frameElement) return Math.max(initialLeftPaneWidth, minLeftPaneWidth);
    return Math.max(minLeftPaneWidth, frameElement.clientWidth - dividerWidth - minRightPaneWidth);
  });
  const constrainedLeftPaneWidth = $derived(
    Math.min(Math.max(leftPaneWidth, minLeftPaneWidth), maxLeftPaneWidth)
  );
  const gridColumns = $derived(
    `${constrainedLeftPaneWidth}px ${dividerWidth}px minmax(${minRightPaneWidth}px, 1fr)`
  );

  function resizeLeftPane(clientX: number) {
    if (!frameElement) return;

    const frameRect = frameElement.getBoundingClientRect();
    leftPaneWidth = Math.min(
      Math.max(clientX - frameRect.left, minLeftPaneWidth),
      frameRect.width - dividerWidth - minRightPaneWidth
    );
  }

  function startResize(event: PointerEvent) {
    if (!frameElement) return;

    event.preventDefault();
    resizeLeftPane(event.clientX);

    function handlePointerMove(moveEvent: PointerEvent) {
      resizeLeftPane(moveEvent.clientX);
    }

    function stopResize() {
      window.removeEventListener('pointermove', handlePointerMove);
      window.removeEventListener('pointerup', stopResize);
    }

    window.addEventListener('pointermove', handlePointerMove);
    window.addEventListener('pointerup', stopResize, { once: true });
  }

  function resizeWithKeyboard(event: KeyboardEvent) {
    if (event.key !== 'ArrowLeft' && event.key !== 'ArrowRight') return;

    event.preventDefault();
    const direction = event.key === 'ArrowLeft' ? -1 : 1;
    const step = event.shiftKey ? 48 : 24;
    leftPaneWidth = Math.min(Math.max(leftPaneWidth + direction * step, minLeftPaneWidth), maxLeftPaneWidth);
  }

  return {
    get frameElement() {
      return frameElement;
    },
    set frameElement(element: HTMLElement | undefined) {
      frameElement = element;
    },
    get gridColumns() {
      return gridColumns;
    },
    get minLeftPaneWidth() {
      return minLeftPaneWidth;
    },
    get maxLeftPaneWidth() {
      return maxLeftPaneWidth;
    },
    get constrainedLeftPaneWidth() {
      return constrainedLeftPaneWidth;
    },
    startResize,
    resizeWithKeyboard
  };
}
