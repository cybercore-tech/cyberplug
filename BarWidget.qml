import QtQuick
import qs.Commons
import qs.Ui

BarWidget {
  id: root
  moduleName: "io.github.darkstardevx.cyberplug"

  // Resolve against this file so cloned/hand-installed copies still find
  // cyberplug-toggle next to the QML — never hardcode ~/.config/omarchy/...
  readonly property string pluginDir: decodeURIComponent(
    String(Qt.resolvedUrl("."))
      .replace(/^file:\/\//, "")
      .replace(/\/+$/, ""))
  readonly property string togglePath: pluginDir + "/cyberplug-toggle"

  function launch() {
    if (!root.bar) return
    root.bar.run(Util.shellQuote(root.togglePath))
  }

  implicitWidth: button.implicitWidth
  implicitHeight: button.implicitHeight

  WidgetButton {
    id: button
    bar: root.bar
    text: "\uf1e6"
    tooltipText: "Cyberplug"
    horizontalMargin: 6
    verticalPadding: 6
    fixedWidth: root.vertical ? root.barSize : Style.space(20)
    fixedHeight: root.barSize
    onPressed: root.launch()
  }
}
