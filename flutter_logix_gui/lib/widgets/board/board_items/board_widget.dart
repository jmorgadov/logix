import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/widgets/board/board_info.dart';
import 'package:flutter_logix_gui/widgets/board/board_items/board_item.dart';

class BoardWidget extends BoardItem {
  const BoardWidget({
    required super.position,
    required super.size,
    required this.child,
    this.onStartMoving,
    this.onMove,
    this.onEndMoving,
    super.margin = EdgeInsets.zero,
  });

  final Widget child;
  final Function()? onStartMoving;
  final Function(Offset delta)? onMove;
  final Function()? onEndMoving;

  @override
  Widget build(BuildContext context, BoardInfo info) {
    return Listener(
      behavior: HitTestBehavior.translucent,
      onPointerMove: (details) {
        final pos = Offset(details.delta.dx, -details.delta.dy) /
            (info.pixelsPerUnit.toDouble() * info.scale);
        onMove?.call(pos);
      },
      child: child,
    );
  }
}
