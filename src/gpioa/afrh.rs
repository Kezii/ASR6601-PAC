#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AfrhSpec>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AfrhSpec>;
#[doc = "pin8 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af8 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af8> for u8 {
    #[inline(always)]
    fn from(variant: Af8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af8 {
    type Ux = u8;
}
impl crate::IsEnum for Af8 {}
#[doc = "Field `AF8` reader - pin8 function selection"]
pub type Af8R = crate::FieldReader<Af8>;
impl Af8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af8> {
        match self.bits {
            0 => Some(Af8::Fun0),
            1 => Some(Af8::Fun1),
            2 => Some(Af8::Fun2),
            3 => Some(Af8::Fun3),
            4 => Some(Af8::Fun4),
            5 => Some(Af8::Fun5),
            6 => Some(Af8::Fun6),
            7 => Some(Af8::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af8::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af8::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af8::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af8::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af8::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af8::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af8::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af8::Fun7
    }
}
#[doc = "Field `AF8` writer - pin8 function selection"]
pub type Af8W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af8>;
impl<'a, REG> Af8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af8::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af8::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af8::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af8::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af8::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af8::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af8::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af8::Fun7)
    }
}
#[doc = "pin9 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af9 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af9> for u8 {
    #[inline(always)]
    fn from(variant: Af9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af9 {
    type Ux = u8;
}
impl crate::IsEnum for Af9 {}
#[doc = "Field `AF9` reader - pin9 function selection"]
pub type Af9R = crate::FieldReader<Af9>;
impl Af9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af9> {
        match self.bits {
            0 => Some(Af9::Fun0),
            1 => Some(Af9::Fun1),
            2 => Some(Af9::Fun2),
            3 => Some(Af9::Fun3),
            4 => Some(Af9::Fun4),
            5 => Some(Af9::Fun5),
            6 => Some(Af9::Fun6),
            7 => Some(Af9::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af9::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af9::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af9::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af9::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af9::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af9::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af9::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af9::Fun7
    }
}
#[doc = "Field `AF9` writer - pin9 function selection"]
pub type Af9W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af9>;
impl<'a, REG> Af9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af9::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af9::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af9::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af9::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af9::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af9::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af9::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af9::Fun7)
    }
}
#[doc = "pin10 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af10 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af10> for u8 {
    #[inline(always)]
    fn from(variant: Af10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af10 {
    type Ux = u8;
}
impl crate::IsEnum for Af10 {}
#[doc = "Field `AF10` reader - pin10 function selection"]
pub type Af10R = crate::FieldReader<Af10>;
impl Af10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af10> {
        match self.bits {
            0 => Some(Af10::Fun0),
            1 => Some(Af10::Fun1),
            2 => Some(Af10::Fun2),
            3 => Some(Af10::Fun3),
            4 => Some(Af10::Fun4),
            5 => Some(Af10::Fun5),
            6 => Some(Af10::Fun6),
            7 => Some(Af10::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af10::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af10::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af10::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af10::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af10::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af10::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af10::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af10::Fun7
    }
}
#[doc = "Field `AF10` writer - pin10 function selection"]
pub type Af10W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af10>;
impl<'a, REG> Af10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af10::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af10::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af10::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af10::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af10::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af10::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af10::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af10::Fun7)
    }
}
#[doc = "pin11 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af11 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af11> for u8 {
    #[inline(always)]
    fn from(variant: Af11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af11 {
    type Ux = u8;
}
impl crate::IsEnum for Af11 {}
#[doc = "Field `AF11` reader - pin11 function selection"]
pub type Af11R = crate::FieldReader<Af11>;
impl Af11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af11> {
        match self.bits {
            0 => Some(Af11::Fun0),
            1 => Some(Af11::Fun1),
            2 => Some(Af11::Fun2),
            3 => Some(Af11::Fun3),
            4 => Some(Af11::Fun4),
            5 => Some(Af11::Fun5),
            6 => Some(Af11::Fun6),
            7 => Some(Af11::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af11::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af11::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af11::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af11::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af11::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af11::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af11::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af11::Fun7
    }
}
#[doc = "Field `AF11` writer - pin11 function selection"]
pub type Af11W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af11>;
impl<'a, REG> Af11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af11::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af11::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af11::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af11::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af11::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af11::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af11::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af11::Fun7)
    }
}
#[doc = "pin12 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af12 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af12> for u8 {
    #[inline(always)]
    fn from(variant: Af12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af12 {
    type Ux = u8;
}
impl crate::IsEnum for Af12 {}
#[doc = "Field `AF12` reader - pin12 function selection"]
pub type Af12R = crate::FieldReader<Af12>;
impl Af12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af12> {
        match self.bits {
            0 => Some(Af12::Fun0),
            1 => Some(Af12::Fun1),
            2 => Some(Af12::Fun2),
            3 => Some(Af12::Fun3),
            4 => Some(Af12::Fun4),
            5 => Some(Af12::Fun5),
            6 => Some(Af12::Fun6),
            7 => Some(Af12::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af12::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af12::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af12::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af12::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af12::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af12::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af12::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af12::Fun7
    }
}
#[doc = "Field `AF12` writer - pin12 function selection"]
pub type Af12W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af12>;
impl<'a, REG> Af12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af12::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af12::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af12::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af12::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af12::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af12::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af12::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af12::Fun7)
    }
}
#[doc = "pin13 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af13 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af13> for u8 {
    #[inline(always)]
    fn from(variant: Af13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af13 {
    type Ux = u8;
}
impl crate::IsEnum for Af13 {}
#[doc = "Field `AF13` reader - pin13 function selection"]
pub type Af13R = crate::FieldReader<Af13>;
impl Af13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af13> {
        match self.bits {
            0 => Some(Af13::Fun0),
            1 => Some(Af13::Fun1),
            2 => Some(Af13::Fun2),
            3 => Some(Af13::Fun3),
            4 => Some(Af13::Fun4),
            5 => Some(Af13::Fun5),
            6 => Some(Af13::Fun6),
            7 => Some(Af13::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af13::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af13::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af13::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af13::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af13::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af13::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af13::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af13::Fun7
    }
}
#[doc = "Field `AF13` writer - pin13 function selection"]
pub type Af13W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af13>;
impl<'a, REG> Af13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af13::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af13::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af13::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af13::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af13::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af13::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af13::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af13::Fun7)
    }
}
#[doc = "pin14 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af14 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af14> for u8 {
    #[inline(always)]
    fn from(variant: Af14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af14 {
    type Ux = u8;
}
impl crate::IsEnum for Af14 {}
#[doc = "Field `AF14` reader - pin14 function selection"]
pub type Af14R = crate::FieldReader<Af14>;
impl Af14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af14> {
        match self.bits {
            0 => Some(Af14::Fun0),
            1 => Some(Af14::Fun1),
            2 => Some(Af14::Fun2),
            3 => Some(Af14::Fun3),
            4 => Some(Af14::Fun4),
            5 => Some(Af14::Fun5),
            6 => Some(Af14::Fun6),
            7 => Some(Af14::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af14::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af14::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af14::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af14::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af14::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af14::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af14::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af14::Fun7
    }
}
#[doc = "Field `AF14` writer - pin14 function selection"]
pub type Af14W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af14>;
impl<'a, REG> Af14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af14::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af14::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af14::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af14::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af14::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af14::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af14::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af14::Fun7)
    }
}
#[doc = "pin15 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af15 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af15> for u8 {
    #[inline(always)]
    fn from(variant: Af15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af15 {
    type Ux = u8;
}
impl crate::IsEnum for Af15 {}
#[doc = "Field `AF15` reader - pin15 function selection"]
pub type Af15R = crate::FieldReader<Af15>;
impl Af15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af15> {
        match self.bits {
            0 => Some(Af15::Fun0),
            1 => Some(Af15::Fun1),
            2 => Some(Af15::Fun2),
            3 => Some(Af15::Fun3),
            4 => Some(Af15::Fun4),
            5 => Some(Af15::Fun5),
            6 => Some(Af15::Fun6),
            7 => Some(Af15::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af15::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af15::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af15::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af15::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af15::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af15::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af15::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af15::Fun7
    }
}
#[doc = "Field `AF15` writer - pin15 function selection"]
pub type Af15W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af15>;
impl<'a, REG> Af15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af15::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af15::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af15::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af15::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af15::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af15::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af15::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af15::Fun7)
    }
}
impl R {
    #[doc = "Bits 0:3 - pin8 function selection"]
    #[inline(always)]
    pub fn af8(&self) -> Af8R {
        Af8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - pin9 function selection"]
    #[inline(always)]
    pub fn af9(&self) -> Af9R {
        Af9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - pin10 function selection"]
    #[inline(always)]
    pub fn af10(&self) -> Af10R {
        Af10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - pin11 function selection"]
    #[inline(always)]
    pub fn af11(&self) -> Af11R {
        Af11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - pin12 function selection"]
    #[inline(always)]
    pub fn af12(&self) -> Af12R {
        Af12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - pin13 function selection"]
    #[inline(always)]
    pub fn af13(&self) -> Af13R {
        Af13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - pin14 function selection"]
    #[inline(always)]
    pub fn af14(&self) -> Af14R {
        Af14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - pin15 function selection"]
    #[inline(always)]
    pub fn af15(&self) -> Af15R {
        Af15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - pin8 function selection"]
    #[inline(always)]
    pub fn af8(&mut self) -> Af8W<'_, AfrhSpec> {
        Af8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - pin9 function selection"]
    #[inline(always)]
    pub fn af9(&mut self) -> Af9W<'_, AfrhSpec> {
        Af9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - pin10 function selection"]
    #[inline(always)]
    pub fn af10(&mut self) -> Af10W<'_, AfrhSpec> {
        Af10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - pin11 function selection"]
    #[inline(always)]
    pub fn af11(&mut self) -> Af11W<'_, AfrhSpec> {
        Af11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - pin12 function selection"]
    #[inline(always)]
    pub fn af12(&mut self) -> Af12W<'_, AfrhSpec> {
        Af12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - pin13 function selection"]
    #[inline(always)]
    pub fn af13(&mut self) -> Af13W<'_, AfrhSpec> {
        Af13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - pin14 function selection"]
    #[inline(always)]
    pub fn af14(&mut self) -> Af14W<'_, AfrhSpec> {
        Af14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - pin15 function selection"]
    #[inline(always)]
    pub fn af15(&mut self) -> Af15W<'_, AfrhSpec> {
        Af15W::new(self, 28)
    }
}
#[doc = "alternate function high register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrhSpec;
impl crate::RegisterSpec for AfrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::Readable for AfrhSpec {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::Writable for AfrhSpec {
    type Safety = crate::Unsafe;
}
