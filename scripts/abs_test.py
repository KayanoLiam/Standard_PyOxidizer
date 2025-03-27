import Standard_PyOxidizer


def main():
    # sp = Standard_PyOxidizer.ByteArray(b'Hello, World!')
    # print(sp)
    sp = Standard_PyOxidizer.ByteArray()
    sp.__init__("Hello, World!")
    print(sp)
    sp.append("ok")
    print(sp)



if '__main__' == __name__:
    main()
