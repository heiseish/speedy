use {
    crate::{
        Context,
        Readable,
        Reader,
        Writable,
        Writer,
    },
    rust_decimal::Decimal,
};

// todo: optimize with copy from slice with edianness

impl<'a, C> Readable<'a, C> for Decimal
where
    C: Context,
{
    #[inline]
    fn read_from<R: Reader<'a, C>>(reader: &mut R) -> Result<Self, C::Error> {
        let mut buffer = [0; 16];
        for i in 0..16 {
            buffer[i] = reader.read_u8()?;
        }
        Ok(Decimal::deserialize(buffer))
    }

    #[inline]
    fn minimum_bytes_needed() -> usize {
        16
    }
}


impl<C> Writable<C> for Decimal
where
    C: Context,
{
    #[inline]
    fn write_to<T: ?Sized + Writer<C>>(&self, writer: &mut T) -> Result<(), C::Error> {
        let bytes = self.serialize();
        for byte in bytes {
            writer.write_u8(byte)?;
        }
        Ok(())
    }

    #[inline]
    fn bytes_needed(&self) -> Result<usize, C::Error> {
        Ok(16)
    }
}
