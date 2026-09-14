#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<IntEnSpec>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<IntEnSpec>;
#[doc = "Field `OPERATION_DONE_INT_EN` reader - Operation done interrupt enable"]
pub type OperationDoneIntEnR = crate::BitReader;
#[doc = "Field `OPERATION_DONE_INT_EN` writer - Operation done interrupt enable"]
pub type OperationDoneIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG_DATA_WAIT_INT_EN` reader - Prog data wait interrupt enable"]
pub type ProgDataWaitIntEnR = crate::BitReader;
#[doc = "Field `PROG_DATA_WAIT_INT_EN` writer - Prog data wait interrupt enable"]
pub type ProgDataWaitIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHBUSY_ERR_INT_EN` reader - Flash busy error interrupt enable"]
pub type FlashbusyErrIntEnR = crate::BitReader;
#[doc = "Field `FLASHBUSY_ERR_INT_EN` writer - Flash busy error interrupt enable"]
pub type FlashbusyErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTION_WR_ERR_INT_EN` reader - Option write error interrupt enable"]
pub type OptionWrErrIntEnR = crate::BitReader;
#[doc = "Field `OPTION_WR_ERR_INT_EN` writer - Option write error interrupt enable"]
pub type OptionWrErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE_ERASE_ERR_INT_EN` reader - Page erase error interrupt enable"]
pub type PageEraseErrIntEnR = crate::BitReader;
#[doc = "Field `PAGE_ERASE_ERR_INT_EN` writer - Page erase error interrupt enable"]
pub type PageEraseErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG_ERR_INT_EN` reader - Prog error interrupt enable"]
pub type ProgErrIntEnR = crate::BitReader;
#[doc = "Field `PROG_ERR_INT_EN` writer - Prog error interrupt enable"]
pub type ProgErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONE_BIT_CORRECT_INT_EN` reader - One bit correct interrupt enable"]
pub type OneBitCorrectIntEnR = crate::BitReader;
#[doc = "Field `ONE_BIT_CORRECT_INT_EN` writer - One bit correct interrupt enable"]
pub type OneBitCorrectIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWO_BIT_ERROR_INT_EN` reader - Two bit error interrupt enable"]
pub type TwoBitErrorIntEnR = crate::BitReader;
#[doc = "Field `TWO_BIT_ERROR_INT_EN` writer - Two bit error interrupt enable"]
pub type TwoBitErrorIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operation done interrupt enable"]
    #[inline(always)]
    pub fn operation_done_int_en(&self) -> OperationDoneIntEnR {
        OperationDoneIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Prog data wait interrupt enable"]
    #[inline(always)]
    pub fn prog_data_wait_int_en(&self) -> ProgDataWaitIntEnR {
        ProgDataWaitIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash busy error interrupt enable"]
    #[inline(always)]
    pub fn flashbusy_err_int_en(&self) -> FlashbusyErrIntEnR {
        FlashbusyErrIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Option write error interrupt enable"]
    #[inline(always)]
    pub fn option_wr_err_int_en(&self) -> OptionWrErrIntEnR {
        OptionWrErrIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Page erase error interrupt enable"]
    #[inline(always)]
    pub fn page_erase_err_int_en(&self) -> PageEraseErrIntEnR {
        PageEraseErrIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Prog error interrupt enable"]
    #[inline(always)]
    pub fn prog_err_int_en(&self) -> ProgErrIntEnR {
        ProgErrIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - One bit correct interrupt enable"]
    #[inline(always)]
    pub fn one_bit_correct_int_en(&self) -> OneBitCorrectIntEnR {
        OneBitCorrectIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Two bit error interrupt enable"]
    #[inline(always)]
    pub fn two_bit_error_int_en(&self) -> TwoBitErrorIntEnR {
        TwoBitErrorIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operation done interrupt enable"]
    #[inline(always)]
    pub fn operation_done_int_en(&mut self) -> OperationDoneIntEnW<'_, IntEnSpec> {
        OperationDoneIntEnW::new(self, 0)
    }
    #[doc = "Bit 2 - Prog data wait interrupt enable"]
    #[inline(always)]
    pub fn prog_data_wait_int_en(&mut self) -> ProgDataWaitIntEnW<'_, IntEnSpec> {
        ProgDataWaitIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Flash busy error interrupt enable"]
    #[inline(always)]
    pub fn flashbusy_err_int_en(&mut self) -> FlashbusyErrIntEnW<'_, IntEnSpec> {
        FlashbusyErrIntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Option write error interrupt enable"]
    #[inline(always)]
    pub fn option_wr_err_int_en(&mut self) -> OptionWrErrIntEnW<'_, IntEnSpec> {
        OptionWrErrIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Page erase error interrupt enable"]
    #[inline(always)]
    pub fn page_erase_err_int_en(&mut self) -> PageEraseErrIntEnW<'_, IntEnSpec> {
        PageEraseErrIntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Prog error interrupt enable"]
    #[inline(always)]
    pub fn prog_err_int_en(&mut self) -> ProgErrIntEnW<'_, IntEnSpec> {
        ProgErrIntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - One bit correct interrupt enable"]
    #[inline(always)]
    pub fn one_bit_correct_int_en(&mut self) -> OneBitCorrectIntEnW<'_, IntEnSpec> {
        OneBitCorrectIntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Two bit error interrupt enable"]
    #[inline(always)]
    pub fn two_bit_error_int_en(&mut self) -> TwoBitErrorIntEnW<'_, IntEnSpec> {
        TwoBitErrorIntEnW::new(self, 8)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnSpec;
impl crate::RegisterSpec for IntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for IntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for IntEnSpec {
    type Safety = crate::Unsafe;
}
