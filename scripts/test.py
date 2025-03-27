import Standard_PyOxidizer
import pytest

def test_creation_from_bytes():
    """测试从bytes创建ByteArray"""
    b = b"hello"
    ba = Standard_PyOxidizer.ByteArray(b)
    assert ba.data == b
    assert len(ba) == 5

def test_creation_from_str():
    """测试从str创建ByteArray"""
    s = "hello"
    ba = Standard_PyOxidizer.ByteArray(s)
    assert ba.data == s.encode('utf-8')
    assert len(ba) == 5

def test_creation_from_bytearray():
    """测试从bytearray创建ByteArray"""
    b = bytearray(b"hello")
    ba = Standard_PyOxidizer.ByteArray(b)
    assert ba.data == b
    assert len(ba) == 5

def test_invalid_creation():
    """测试无效输入类型"""
    with pytest.raises(TypeError):
        Standard_PyOxidizer.ByteArray(123)  # 不支持的数字类型

def test_to_bytes():
    """测试转换为bytes"""
    ba = Standard_PyOxidizer.ByteArray(b"hello")
    py_bytes = ba.to_bytes()
    assert isinstance(py_bytes, bytes)
    assert py_bytes == b"hello"

def test_to_bytearray():
    """测试转换为bytearray"""
    ba = Standard_PyOxidizer.ByteArray(b"hello")
    py_bytearray = ba.to_bytearray()
    assert isinstance(py_bytearray, bytearray)
    assert py_bytearray == bytearray(b"hello")

def test_repr():
    """测试__repr__方法"""
    # 可打印字符串
    ba = Standard_PyOxidizer.ByteArray("hello")
    assert repr(ba) == "ByteArray('hello')"
    
    # 不可打印字节
    ba = Standard_PyOxidizer.ByteArray(b'\x01\x02\x03')
    assert repr(ba) == "ByteArray('\x01\x02\x03')"

def test_str():
    """测试__str__方法"""
    ba = Standard_PyOxidizer.ByteArray("hello")
    assert str(ba) == "ByteArray('hello')"

def test_len():
    """测试__len__方法"""
    ba = Standard_PyOxidizer.ByteArray(b"hello")
    assert len(ba) == 5
    ba = Standard_PyOxidizer.ByteArray(b"")
    assert len(ba) == 0

def test_getitem():
    """测试__getitem__方法"""
    ba = Standard_PyOxidizer.ByteArray(b"hello")
    assert ba[0] == ord('h')
    assert ba[-1] == ord('o')
    assert ba[1] == ord('e')

def test_getitem_out_of_range():
    """测试索引越界"""
    ba = Standard_PyOxidizer.ByteArray(b"hello")
    with pytest.raises(TypeError):
        _ = ba[5]  # 正向越界
    with pytest.raises(TypeError):
        _ = ba[-6]  # 负向越界

def test_add():
    """测试__add__方法"""
    ba1 = Standard_PyOxidizer.ByteArray(b"hel")
    ba2 = Standard_PyOxidizer.ByteArray(b"lo")
    result = ba1 + ba2
    assert result.data == b"hello"

def test_append():
    """测试append方法"""
    ba1 = Standard_PyOxidizer.ByteArray(b"hel")
    ba2 = Standard_PyOxidizer.ByteArray(b"lo")
    ba1.append(ba2)
    assert ba1.data == b"hello"

def test_slice():
    """测试slice方法"""
    ba = Standard_PyOxidizer.ByteArray(b"hello")
    # 正常切片
    sliced = ba.slice(1, 4, None)
    assert sliced.data == b"ell"
    # 负索引
    sliced = ba.slice(-4, -1, None)
    assert sliced.data == b"ell"
    # 省略start
    sliced = ba.slice(None, 3, None)
    assert sliced.data == b"hel"
    # 省略end
    sliced = ba.slice(2, None, None)
    assert sliced.data == b"llo"
    # 步长
    sliced = ba.slice(0, None, 2)
    assert sliced.data == b"hlo"

def test_slice_invalid_step():
    """测试无效步长"""
    ba = Standard_PyOxidizer.ByteArray(b"hello")
    with pytest.raises(TypeError):
        ba.slice(0, 5, 0)  # 步长为0

if __name__ == "__main__":
    pytest.main(["-v", __file__])
