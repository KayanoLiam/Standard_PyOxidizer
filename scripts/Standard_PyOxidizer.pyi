from typing import Optional, Union

def abs_i32(x: int) -> int: ...
def abs_i64(x: int) -> int: ...
def abs_i128(x: int) -> int: ...
def abs_f32(x: float) -> float: ...
def abs_f64(x: float) -> float: ...
def abs_complex(real: float, imag: float) -> float: ...
def ascii(x: str) -> str: ...
def bin(x: int, prefix: bool = True) -> str: ...

class ByteArray:
    """A Python-compatible byte array implementation."""
    
    data: bytes
    
    def __init__(self, py_data: Union[bytes, bytearray, str]) -> None:
        """Initialize from bytes, bytearray or str.
        
        Args:
            py_data: Input data to convert to ByteArray
        Raises:
            TypeError: If input type is not supported
        """
        ...
    
    def to_bytes(self) -> bytes:
        """Convert to Python bytes object."""
        ...
    
    def to_bytearray(self) -> bytearray:
        """Convert to Python bytearray."""
        ...
    
    def __repr__(self) -> str:
        """Return formal string representation."""
        ...
    
    def __str__(self) -> str:
        """Return informal string representation."""
        ...
    
    def __len__(self) -> int:
        """Return length of the byte array."""
        ...
    
    def __getitem__(self, idx: int) -> int:
        """Get byte at index.
        
        Args:
            idx: Index (can be negative for reverse indexing)
        Raises:
            IndexError: If index is out of range
        """
        ...
    
    def __add__(self, other: 'ByteArray') -> 'ByteArray':
        """Concatenate two ByteArrays."""
        ...
    
    def append(self, other: 'ByteArray') -> None:
        """Append another ByteArray to this one."""
        ...
    
    def slice(
        self,
        start: Optional[int] = None,
        end: Optional[int] = None,
        step: Optional[int] = None
    ) -> 'ByteArray':
        """Return sliced ByteArray.
        
        Args:
            start: Start index (None means beginning)
            end: End index (None means end)
            step: Step size (None means 1)
        Raises:
            ValueError: If step is 0
        """
        ...
