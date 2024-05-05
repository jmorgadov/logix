import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/widgets/board/board_info.dart';
import 'package:flutter_logix_gui/widgets/board/board_items/board_item.dart';

typedef OnBoardDrag = void Function(BoardInfo);
typedef OnBoardPointerMove = void Function(BoardInfo, Offset);
typedef OnBoardPointerDown = void Function(BoardInfo, Offset);
typedef OnBoardPointerUp = void Function(BoardInfo, Offset);
typedef OnBoardScale = void Function(BoardInfo);

class Board extends StatelessWidget {
  const Board({
    super.key,
    this.pixelsPerUnit = 40,
    this.showGrid = true,
    this.onPointerMove,
    this.onPointerDown,
    this.onPointerUp,
    this.onPonterDrag,
    this.onPointerScale,
    this.children,
    this.isDiscrete = false,
  });

  final int pixelsPerUnit;
  final bool showGrid;
  final OnBoardPointerMove? onPointerMove;
  final OnBoardDrag? onPonterDrag;
  final OnBoardScale? onPointerScale;
  final OnBoardPointerDown? onPointerDown;
  final OnBoardPointerUp? onPointerUp;
  final List<BoardItem>? children;
  final bool isDiscrete;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(builder: (context, constraints) {
      final width = constraints.maxWidth;
      final height = constraints.maxHeight;

      return SizedBoard(
        width: width,
        height: height,
        pixelsPerUnit: pixelsPerUnit,
        showGrid: showGrid,
        onPointerMove: onPointerMove,
        onPointerDown: onPointerDown,
        onPointerUp: onPointerUp,
        onPointerDrag: onPonterDrag,
        onPointerScale: onPointerScale,
        children: children,
        isDiscrete: isDiscrete,
      );
    });
  }
}

class SizedBoard extends StatefulWidget {
  const SizedBoard({
    super.key,
    required this.width,
    required this.height,
    this.pixelsPerUnit = 40,
    this.showGrid = true,
    this.onPointerMove,
    this.onPointerDown,
    this.onPointerUp,
    this.onPointerDrag,
    this.onPointerScale,
    this.children,
    this.isDiscrete = false,
  });

  final double width;
  final double height;
  final int pixelsPerUnit;
  final bool showGrid;
  final List<BoardItem>? children;
  final OnBoardPointerMove? onPointerMove;
  final OnBoardPointerDown? onPointerDown;
  final OnBoardPointerUp? onPointerUp;
  final OnBoardDrag? onPointerDrag;
  final OnBoardScale? onPointerScale;
  final bool isDiscrete;

  @override
  State<SizedBoard> createState() => _BoardState();
}

class _BoardState extends State<SizedBoard> {
  Offset _offset = Offset.zero;
  double _scale = 1;
  bool _isDragging = false;
  bool _isSelecting = false;

  Offset selectionStart = Offset.zero;
  Offset selectionEnd = Offset.zero;

  get selectionRect => Rect.fromPoints(selectionStart, selectionEnd);

  get bounds => Rect.fromLTWH(
      -width / 2 - _offset.dx, -height / 2 - _offset.dy, width, height);
  get height => widget.height;
  get width => widget.width;

  late BoardInfo info;

  @override
  void initState() {
    super.initState();
    info = BoardInfo(
      pixelsPerUnit: widget.pixelsPerUnit,
      bounds: bounds,
      offset: _offset,
      scale: _scale,
      selectionRect:
          _isSelecting ? Rect.fromPoints(selectionStart, selectionEnd) : null,
      isDiscrete: widget.isDiscrete,
    );
  }

  void _updateInfo() {
    info = BoardInfo(
      pixelsPerUnit: widget.pixelsPerUnit,
      bounds: bounds,
      offset: _offset,
      scale: _scale,
      selectionRect:
          _isSelecting ? Rect.fromPoints(selectionStart, selectionEnd) : null,
      isDiscrete: widget.isDiscrete,
    );
  }

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Positioned.fill(
          child: CustomPaint(
            painter: BoardGridPainter(
              pixelsPerUnit: widget.pixelsPerUnit,
              showGrid: widget.showGrid,
              scale: _scale,
              offset: _offset + Offset(width / 2, height / 2),
              width: width,
              height: height,
            ),
          ),
        ),
        Positioned.fill(
          child: Listener(
            onPointerSignal: (details) {
              if (details is PointerScrollEvent) {
                setState(() {
                  if (details.scrollDelta.dy < 0) {
                    _scale *= 1.1;
                  } else {
                    _scale /= 1.1;
                  }
                  final pos = localToBoard(details.localPosition);

                  final invPos = Offset(
                    pos.dx,
                    -pos.dy,
                  );

                  if (details.scrollDelta.dy < 0) {
                    _offset -= invPos * .1 * _scale;
                  } else {
                    _offset += invPos * .1 * _scale;
                  }

                  _updateInfo();
                  widget.onPointerScale?.call(info);
                });
              }
            },
            onPointerDown: (details) {
              if (details.buttons == kMiddleMouseButton) {
                setState(() {
                  _isDragging = true;
                });
              }
              if (details.buttons == kPrimaryMouseButton) {
                setState(() {
                  _isSelecting = true;
                  selectionStart = localToBoard(details.localPosition);
                  selectionEnd = selectionStart;
                });
              }
              widget.onPointerDown
                  ?.call(info, localToBoard(details.localPosition));
            },
            onPointerMove: (details) {
              if (_isDragging) {
                setState(() {
                  _offset += details.delta;
                  _updateInfo();
                  widget.onPointerDrag?.call(info);
                });
              }
              if (_isSelecting) {
                setState(() {
                  selectionEnd = localToBoard(details.localPosition);
                });
              }
              widget.onPointerMove
                  ?.call(info, localToBoard(details.localPosition));
            },
            onPointerUp: (details) {
              if (_isDragging) {
                setState(() {
                  _isDragging = false;
                });
              }
              if (_isSelecting) {
                setState(() {
                  _isSelecting = false;
                });
              }
              widget.onPointerUp
                  ?.call(info, localToBoard(details.localPosition));
            },
            child: MouseRegion(
              onHover: (details) {
                widget.onPointerMove
                    ?.call(info, localToBoard(details.localPosition));
              },
            ),
          ),
        ),
        if (widget.children != null)
          ...widget.children!.map((item) {
            return item.internalBuild(context, info);
          }),
        Positioned.fill(
          child: IgnorePointer(
            ignoring: true,
            child: CustomPaint(
              painter: BoardSelectionPainter(
                selection: _isSelecting
                    ? Rect.fromPoints(boardToLocal(selectionStart),
                        boardToLocal(selectionEnd))
                    : null,
              ),
            ),
          ),
        ),
      ],
    );
  }

  Offset boardToLocal(Offset canvasPos) {
    canvasPos = Offset(
      canvasPos.dx,
      -canvasPos.dy,
    );
    final globalPos = canvasPos * _scale + _offset;
    return globalPos + Offset(width / 2, height / 2);
  }

  Offset localToBoard(Offset localPos) {
    final globalPos = localPos - Offset(width / 2, height / 2);
    var pos = globalPos - _offset;

    pos = Offset(
      pos.dx / _scale,
      pos.dy / _scale,
    );

    pos = Offset(
      pos.dx,
      -pos.dy,
    );

    return pos;
  }
}

class BoardGridPainter extends CustomPainter {
  BoardGridPainter({
    required this.pixelsPerUnit,
    required this.scale,
    required this.offset,
    required this.width,
    required this.height,
    required this.showGrid,
  });

  final int pixelsPerUnit;
  final double scale;
  final Offset offset;
  final double width;
  final double height;
  final bool showGrid;

  @override
  void paint(Canvas canvas, Size size) {
    if (showGrid) {
      _drawGrid(canvas);
    }
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    if (oldDelegate is BoardGridPainter) {
      return oldDelegate.scale != scale ||
          oldDelegate.offset != offset ||
          oldDelegate.width != width ||
          oldDelegate.height != height;
    }
    return true;
  }

  void _drawGrid(Canvas canvas) {
    final paint = Paint()
      ..color = Colors.grey.withOpacity(.3)
      ..strokeWidth = scale * 0.3;

    final gridSizePx = pixelsPerUnit * scale;
    final offsetX = offset.dx % gridSizePx;
    final offsetY = offset.dy % gridSizePx;

    for (var x = offsetX; x < width; x += gridSizePx) {
      canvas.drawLine(
        Offset(x, 0),
        Offset(x, height),
        paint,
      );
    }

    for (var y = offsetY; y < height; y += gridSizePx) {
      canvas.drawLine(
        Offset(0, y),
        Offset(width, y),
        paint,
      );
    }
  }
}

class BoardSelectionPainter extends CustomPainter {
  BoardSelectionPainter({
    this.selection,
  });

  final Rect? selection;

  @override
  void paint(Canvas canvas, Size size) {
    if (selection != null) {
      final paint = Paint()
        ..color = Colors.blue.withOpacity(.2)
        ..style = PaintingStyle.fill;

      canvas.drawRect(selection!, paint);
    }
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    if (oldDelegate is BoardSelectionPainter) {
      return oldDelegate.selection != selection;
    }
    return true;
  }
}
